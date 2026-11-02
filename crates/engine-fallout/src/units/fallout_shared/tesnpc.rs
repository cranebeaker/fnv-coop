//! `fallout shared/tesnpc.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESNPC` is the form of a non-player character (form type `0x2a`). It
//! inherits `TESActorBase` and, through it, a long list of components
//! (`TESActorBaseData`, `TESContainer`, `BGSTouchSpellForm`, `TESSpellList`,
//! `TESAIForm`, `TESHealthForm`, `TESAttributes`, `TESAnimation`,
//! `TESFullName`, `TESModel`, `TESScriptableForm`, `BGSDestructibleObjectForm`),
//! then `TESRaceForm` and its own fields. Every field after `TESForm`'s
//! sits `0x10` lower than in the Xbox PDB, and from `RaceFaceOffsetCoord` on
//! the PC class is `0x20` bytes larger there (the PC face-gen coordinate is
//! four `0x20`-byte matrices; the PDB has `0x60` bytes), so the fields from
//! `pAlternateFaceOffsetCoord` on sit `0x10` higher than in the PDB. The PC
//! class is `0x20c` bytes (the size `CreateFormOfType` allocates).
//!
//! Notes for the next session (this file is translated in blocks of 40
//! functions in address order):
//! - Session 1 (b0016) holds the constructor `00601170` to the function at
//!   `00605d50`. Session 2 (b0016) holds `TESNPC::ReplaceRefModel` `00605d70`
//!   to `0060b1f0` (the block starts at the line "Session 2" below). Session 3
//!   (b0016) holds `0060b210` to `0060bed0`, the end of the unit (the block
//!   starts at the line "Session 3" below): nothing is left to translate here.
//! - A pushed word that stays on the stack across a nested call belongs to the
//!   OUTER call (`PUSH a; CALL getter; MOV ECX,EAX; CALL method` passes `a` to
//!   `method`, and a getter whose `RET` has no number takes nothing): check the
//!   `RET n` of every callee. Likewise the `PUSH 0` before `vcall(actor, 0x1f4)`
//!   is the last word of `008bb520`, not an argument of the virtual.
//! - The `Activate` virtual (`00607990`) is translated block by block as the
//!   methods of `Activation` (one per jump target of the exe, named by
//!   address); read the disassembly next to it when changing anything.
//! - The face-gen coordinate of an NPC is four matrices of `0x20` bytes
//!   (`[sex-or-race][shape/texture]`: index `i * 0x40 + j * 0x20`). The
//!   exe's `00601800` returns the alternate coordinate when
//!   `pAlternateFaceOffsetCoord` is set and `RaceFaceOffsetCoord` otherwise;
//!   it sits in no unit, so it is called by address. `006529a0(source,
//!   destination, 0, 0)` (`BSFaceGenManager::CopyFaceGenCoord`, cdecl, four
//!   words) copies from its FIRST argument into its second.
//! - Many one-line accessors the compiler emitted next to the functions
//!   (`00726070` returns the word at `+4`, `007af430` the word at `+0x20`,
//!   `00401170` the form type byte, ...) belong to other units and are called
//!   by address like every other callee.
//! - The decompiler hangs the pushed words of a cdecl callee on the nearest
//!   thiscall (`PUSH 0; PUSH 0; CALL 00601800` pushes the last two words of
//!   `006529a0`, not arguments of `00601800`): all calls here were read from
//!   the disassembly.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::*;

// ---------------------------------------------------------------------------
// Layout

layout! {
    /// `TESNPC` (Xbox PDB), `0x20c` bytes on PC. Offsets are the PC ones (see
    /// the module notes); only the fields the translations use are listed.
    pub struct TESNPC: 0x20c {
        /// `TESForm::cFormType` (Xbox PDB): `0x2a` for an NPC.
        0x04 cFormType: u8,
        /// `TESForm::iFormFlags` (Xbox PDB).
        0x08 iFormFlags: u32,
        /// `TESForm::iFormID` (Xbox PDB).
        0x0C iFormID: u32,
        /// `TESActorBaseData::actorData.iActorBaseFlags` (Xbox PDB), PC
        /// `+0x34`: the `TESActorBaseData` component starts at `+0x30`.
        0x34 iActorBaseFlags: u32,
        /// `TESHealthForm::iHealth` (Xbox PDB); the component starts at `+0xb0`.
        0xB4 iHealth: u32,
        /// `TESRaceForm::pFormRace` (Xbox PDB); the component starts at `+0x10c`.
        0x110 pFormRace: Ptr,
        /// `data` (Xbox PDB, `NPC_DATA`, `0x1c` bytes: `cSkill[14]`, then
        /// `cOffset[14]`).
        0x114 data: Inline<NpcData>,
        /// `pCl` (Xbox PDB): the NPC's `TESClass *` (a form id until `InitItem`
        /// resolves it).
        0x130 pCl: Ptr,
        /// `RaceFaceOffsetCoord` (Xbox PDB): four `0x20`-byte matrices on PC.
        0x134 RaceFaceOffsetCoord: Inline<FaceGenCoord>,
        /// `pAlternateFaceOffsetCoord` (Xbox PDB).
        0x1B4 pAlternateFaceOffsetCoord: Ptr,
        /// `pHair` (Xbox PDB): `TESHair *`.
        0x1B8 pHair: Ptr,
        /// `fHairLength` (Xbox PDB).
        0x1BC fHairLength: f32,
        /// `pEyeColor` (Xbox PDB): `TESEyes *`.
        0x1C0 pEyeColor: Ptr,
        /// `spHeadBiped` (Xbox PDB): `NiPointer<BSFaceGenNiNode>`.
        0x1C4 spHeadBiped: Ptr,
        /// `spHeadSkinned` (Xbox PDB): `NiPointer<BSFaceGenNiNode>`.
        0x1C8 spHeadSkinned: Ptr,
        /// `spBodyModTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0x1CC spBodyModTexture: Ptr,
        /// `sLastRaceFaceNum` (Xbox PDB).
        0x1D0 sLastRaceFaceNum: u16,
        /// `pCombatStyle` (Xbox PDB): `TESCombatStyle *` (a form id until
        /// `InitItem` resolves it).
        0x1D4 pCombatStyle: Ptr,
        /// `iHairColor` (Xbox PDB).
        0x1D8 iHairColor: u32,
        /// `listHeadParts` (Xbox PDB): `BSSimpleList<BGSHeadPart *>`.
        0x1DC listHeadParts: Inline<BSSimpleList>,
        /// `eBloodImpactMaterial` (Xbox PDB).
        0x1E4 eBloodImpactMaterial: u32,
        /// `iFileOffset` (Xbox PDB).
        0x1E8 iFileOffset: u32,
        /// `pOriginalRace` (Xbox PDB): `TESRace *`.
        0x1EC pOriginalRace: Ptr,
        /// `pFaceNPC` (Xbox PDB): `TESNPC *`.
        0x1F0 pFaceNPC: Ptr,
        /// `fHeight` (Xbox PDB).
        0x1F4 fHeight: f32,
        /// `fWeight` (Xbox PDB).
        0x1F8 fWeight: f32,
        /// `FaceGenUndoStates` (Xbox PDB): `NiTPrimitiveArray<FaceGenUndo *>`.
        0x1FC FaceGenUndoStates: Inline<NiTArray>,
    }

    /// `NPC_DATA` (Xbox PDB), `0x1c` bytes.
    pub struct NpcData: 0x1c {
        /// `cSkill` (Xbox PDB): 14 bytes.
        0x00 cSkill: u8,
        /// `cOffset` (Xbox PDB): 14 bytes.
        0x0E cOffset: u8,
    }

    /// One face-gen coordinate: four `0x20`-byte matrices (`FR2MatrixVTC`).
    pub struct FaceGenCoord: 0x80 {
        0x00 matrices: u8,
    }
}

// Offsets of the components inside a `TESNPC` (PC).
/// `TESActorBaseData` (vtable pointer at `+0x30`).
pub(crate) const COMPONENT_ACTOR_BASE_DATA: u32 = 0x30;
/// `TESContainer`.
pub(crate) const COMPONENT_CONTAINER: u32 = 0x64;
/// `BGSTouchSpellForm`.
pub(crate) const COMPONENT_TOUCH_SPELL: u32 = 0x70;
/// `TESSpellList`.
pub(crate) const COMPONENT_SPELL_LIST: u32 = 0x7c;
/// `TESAIForm`.
pub(crate) const COMPONENT_AI_FORM: u32 = 0x90;
/// `TESHealthForm`.
pub(crate) const COMPONENT_HEALTH: u32 = 0xb0;
/// `TESAttributes`.
pub(crate) const COMPONENT_ATTRIBUTES: u32 = 0xb8;
/// `TESAnimation`.
pub(crate) const COMPONENT_ANIMATION: u32 = 0xc4;
/// `TESFullName`.
pub(crate) const COMPONENT_FULL_NAME: u32 = 0xd0;
/// `TESModel`.
pub(crate) const COMPONENT_MODEL: u32 = 0xdc;
/// `TESScriptableForm`.
pub(crate) const COMPONENT_SCRIPTABLE: u32 = 0xf4;
/// `BGSDestructibleObjectForm`.
pub(crate) const COMPONENT_DESTRUCTIBLE: u32 = 0x104;
/// `TESRaceForm`.
pub(crate) const COMPONENT_RACE: u32 = 0x10c;

// ---------------------------------------------------------------------------
// Exe data and helper functions of other units

/// The data handler singleton pointer (`011c3f2c`, `TESDataHandler`).
pub(crate) const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The player character singleton pointer (`011dea3c`).
pub(crate) const PLAYER_SINGLETON: u32 = 0x011d_ea3c;
/// The save-load object pointer (`011de45c`, `TESSaveLoadGame`).
pub(crate) const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `MOV AL,0` on the save-load object (false on PC).
pub(crate) const SAVE_LOAD_UNAVAILABLE: u32 = 0x0047_c850;
/// The `double` `0.0` the float tests compare with.
pub(crate) const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The static `BSSimpleList<TESNPC *>` of character-creation face presets
/// (`TESNPC::CharGenPresetFaceList`, Xbox PDB).
pub(crate) const CHARGEN_PRESET_FACE_LIST: u32 = 0x011c_b764;

/// Type descriptors `__RTDynamicCast` takes: `TESForm` and the targets.
pub(crate) const TYPE_TES_FORM: u32 = 0x0118_3028;
pub(crate) const TYPE_TES_CLASS: u32 = 0x0118_6424;
pub(crate) const TYPE_TES_COMBAT_STYLE: u32 = 0x0118_627c;
pub(crate) const TYPE_TES_NPC: u32 = 0x0118_3a1c;
/// `__RTDynamicCast(object, vfDelta, sourceType, targetType, isReference)`.
pub(crate) const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `memcpy(destination, source, size)` (the exe's wrapper, cdecl).
pub(crate) const MEMCPY: u32 = 0x0040_1460;
/// `memcmp(a, b, size)` (`00ec4835`, cdecl).
pub(crate) const MEMCMP: u32 = 0x00ec_4835;
/// `strcmp(a, b)` (the wrapper around `00ec6da0`, cdecl).
pub(crate) const STRCMP: u32 = 0x0040_8b20;
/// `_eh_vector_constructor_iterator_(array, size, count, constructor,
/// destructor)` (stdcall).
pub(crate) const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;

/// The word at `+4` of `this` (`00726070`, the list-node `next` getter).
pub(crate) const GET_WORD_AT_4: u32 = 0x0072_6070;
/// The word at `+0x20` of `this` (`007af430`: `TESObject::nod_lpNext` and
/// `TESObjectREFR::baseForm` share it).
pub(crate) const GET_WORD_AT_20: u32 = 0x007a_f430;
/// `TESForm::GetFormType` folded accessor: the byte at `+4`.
pub(crate) const GET_FORM_TYPE: u32 = 0x0040_1170;
/// `this->flags & mask` on a `TESActorBaseData` (`00461580`, thiscall with
/// the mask on the stack).
pub(crate) const TEST_ACTOR_FLAGS: u32 = 0x0046_1580;
/// `TESActorBase::GetSex` (Xbox PDB): 1 when the female flag (bit 0) of a
/// `TESNPC` is set, 0 when not, -1 when it is not an NPC.
pub(crate) const GET_SEX: u32 = 0x005f_0cc0;
/// The combat style getter/setter slots of `TESActorBase`.
pub(crate) const VSLOT_GET_COMBAT_STYLE: u32 = 0x188;
pub(crate) const VSLOT_SET_COMBAT_STYLE: u32 = 0x18c;
/// `TESBoundObject`/`TESForm` virtual slot `IsAutoCalc` (`0x144`).
pub(crate) const VSLOT_IS_AUTO_CALC: u32 = 0x144;
/// Virtual slot `0x130`: the form name (a `const char *`; the base `TESForm`
/// version returns the empty string).
pub(crate) const VSLOT_GET_FORM_NAME: u32 = 0x130;
/// `TESActorBaseData::GetBloodImpactMaterial` / `SetBloodImpactMaterial`.
pub(crate) const VSLOT_GET_BLOOD_IMPACT_MATERIAL: u32 = 0x58;
pub(crate) const VSLOT_SET_BLOOD_IMPACT_MATERIAL: u32 = 0x5c;

/// Face-gen helpers (`BSFaceGenManager`, Xbox PDB).
/// `GetFaceGenCoord()`: the default coordinate (no arguments).
pub(crate) const FACEGEN_DEFAULT_COORD: u32 = 0x0065_21e0;
/// `InitFaceGenCoord(coord)` (cdecl, one word).
pub(crate) const FACEGEN_INIT_COORD: u32 = 0x0065_21f0;
/// `CopyFaceGenCoord(source, destination, 0, 0)` (cdecl, four words).
pub(crate) const FACEGEN_COPY_COORD: u32 = 0x0065_29a0;
/// `FaceGenCoordsDiffer(a, b)` (cdecl, two words): true when they differ.
pub(crate) const FACEGEN_COORDS_DIFFER: u32 = 0x0065_2900;
/// `BlendFaceGenCoords(raceCoord, npcCoord, destination, flag, scale)`
/// (cdecl, five words).
pub(crate) const FACEGEN_BLEND_COORDS: u32 = 0x0065_2af0;
/// The face-gen coordinate of an NPC (`00601800`, thiscall): `pAlternateFaceOffsetCoord`
/// when it is set, otherwise the `RaceFaceOffsetCoord` embedded at `+0x134`.
pub(crate) const NPC_FACE_COORD_POINTER: u32 = 0x0060_1800;
/// `&listHeadParts`, `fHairLength`, `pEyeColor`, `iHairColor` accessors.
pub(crate) const NPC_GET_HEAD_PARTS: u32 = 0x0060_2170;
pub(crate) const NPC_GET_HAIR_LENGTH: u32 = 0x0060_2130;
pub(crate) const NPC_GET_EYE_COLOR: u32 = 0x0060_2150;
pub(crate) const NPC_GET_HAIR_COLOR: u32 = 0x0041_69d0;
/// `TESRaceForm` getter of `pFormRace` (`004ac110`).
pub(crate) const NPC_GET_RACE: u32 = 0x004a_c110;
/// `TESNPC::pCl` getter (`00502430`).
pub(crate) const NPC_GET_CLASS: u32 = 0x0050_2430;
/// `&NPC_DATA` (`00634890`: `this + 0x114`).
pub(crate) const NPC_GET_DATA: u32 = 0x0063_4890;

// ---------------------------------------------------------------------------
// Construction

/// The vtable pointers the constructor stores: (offset in the object, vtable
/// address). Offset 0 is `TESNPC`'s own table; the others are the component
/// tables (see the `COMPONENT_*` offsets).
const VTABLE_STORES: [(u32, u32); 15] = [
    (0x00, 0x0104_a2f4),
    (0x30, 0x0104_a284),
    (0x64, 0x0104_a270),
    (0x70, 0x0104_a25c),
    (0x7c, 0x0104_a23c),
    (0x90, 0x0104_a21c),
    (0xb0, 0x0104_a204),
    (0xb8, 0x0104_a1f0),
    (0xc4, 0x0104_a1dc),
    (0xd0, 0x0104_a1c8),
    (0xdc, 0x0104_a1a4),
    (0xf4, 0x0104_a190),
    (0x100, 0x0104_a160),
    (0x104, 0x0104_a14c),
    (0x10c, 0x0104_a138),
];

// Translated from 00601170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::TESNPC` (Xbox PDB): constructs the `TESActorBase` and
/// `TESRaceForm` bases, stores the vtables, constructs the four face-gen
/// matrices (`0x20` bytes each), the three `NiPointer`s, the head-part list
/// and the undo-state array, then runs `InitializeData` (`00601570`) and
/// seeds the AI form (`0x32`, `0`, `2`, `0x32` through `0047ef50`,
/// `0047ee00`, `0047eed0`, `0047ef00`). The compiler's exception-unwinding
/// frame is not translated.
pub fn tesnpc_tesnpc(e: &mut Engine, this: Ptr<TESNPC>) -> Ptr<TESNPC> {
    let base = this.addr();
    e.call(0x005f_75e0, &args![this]);
    e.call(0x0048_b800, &args![base + COMPONENT_RACE]);
    for (offset, vtable) in VTABLE_STORES {
        e.mem.set_u32(base + offset, vtable);
    }
    // The four `FR2MatrixVTC` of the race coordinate: 4 x 0x20 bytes.
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![base + 0x134, 0x20u32, 4u32, 0x0044_9610u32, 0x0044_9680u32],
    );
    e.call(0x0063_3c90, &args![base + 0x1c4, 0u32]);
    e.call(0x0063_3c90, &args![base + 0x1c8, 0u32]);
    e.call(0x0063_3c90, &args![base + 0x1cc, 0u32]);
    e.call(0x0096_a2d0, &args![base + 0x1dc]);
    e.call(0x0060_b9e0, &args![base + 0x1fc, 0u32, 1u32]);
    e.call(0x004f_15a0, &args![this, 0x2au32]);
    tesnpc_initialize_data(e, this);
    e.call(0x0047_ef50, &args![base + COMPONENT_AI_FORM, 0x32u32, 0u32]);
    e.call(0x0047_ee00, &args![base + COMPONENT_AI_FORM, 0u32, 0u32]);
    e.call(0x0047_eed0, &args![base + COMPONENT_AI_FORM, 2u32, 0u32]);
    e.call(0x0047_ef00, &args![base + COMPONENT_AI_FORM, 0x32u32, 0u32]);
    e.set(this, TESNPC::iFileOffset, 0);
    this
}

// Translated from 00601570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::InitializeData` (virtual `0x14`, Xbox PDB name from the
/// vtable): resets the NPC's own fields: weight and height `0.0`, hair,
/// hair length, eye colour, head nodes (cleared through `NiPointer`
/// assignment unless the save-load object says otherwise), last race face
/// number `0xff`, class and combat style null, hair colour `0x19324b`,
/// no alternate coordinate; copies the default face-gen coordinate into the
/// NPC's, resizes every non-empty matrix of the race coordinate, sets the 14
/// skills to 5 and their offsets to 0, the blood impact material to 6, and
/// clears the original race and face NPC; finally `005f7b50(1, 0)` on the
/// actor-base component and `006ecd40(0x32)` on the health one.
pub fn tesnpc_initialize_data(e: &mut Engine, this: Ptr<TESNPC>) {
    let base = this.addr();
    e.set(this, TESNPC::fWeight, 0.0);
    e.set(this, TESNPC::fHeight, 0.0);
    e.set(this, TESNPC::pHair, Ptr::NULL);
    e.set(this, TESNPC::fHairLength, 0.0);
    e.set(this, TESNPC::pEyeColor, Ptr::NULL);
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if !e.call(SAVE_LOAD_UNAVAILABLE, &args![save_load]).bool() {
        let cleared = e.call(0x0066_b0d0, &args![base + 0x1c8, 0u32]).u32();
        e.call(0x006e_5cc0, &args![base + 0x1c4, cleared]);
        e.call(0x0066_b0d0, &args![base + 0x1cc, 0u32]);
    }
    e.set(this, TESNPC::sLastRaceFaceNum, 0xff);
    e.set(this, TESNPC::pCl, Ptr::NULL);
    e.set(this, TESNPC::pCombatStyle, Ptr::NULL);
    e.set(this, TESNPC::iHairColor, 0x0019_324b);
    e.set(this, TESNPC::pAlternateFaceOffsetCoord, Ptr::NULL);
    let own_coord = e.call(NPC_FACE_COORD_POINTER, &args![this]).u32();
    let default_coord = e.call(FACEGEN_DEFAULT_COORD, &args![]).u32();
    e.call(
        FACEGEN_COPY_COORD,
        &args![default_coord, own_coord, 0u32, 0u32],
    );
    for i in 0..2u32 {
        for j in 0..2u32 {
            let matrix = base + 0x134 + i * 0x40 + j * 0x20;
            if e.call(0x0096_11e0, &args![matrix]).u32() != 0 {
                e.call(0x0060_b3b0, &args![matrix]);
            }
        }
    }
    for k in 0..14u32 {
        e.mem.set_u8(base + 0x114 + k, 5);
        e.mem.set_u8(base + 0x122 + k, 0);
    }
    e.set(this, TESNPC::eBloodImpactMaterial, 6);
    e.set(this, TESNPC::pOriginalRace, Ptr::NULL);
    e.set(this, TESNPC::pFaceNPC, Ptr::NULL);
    e.call(
        0x005f_7b50,
        &args![base + COMPONENT_ACTOR_BASE_DATA, 1u32, 0u32],
    );
    e.call(0x006e_cd40, &args![base + COMPONENT_HEALTH, 0x32u32]);
}

// Translated from 006031e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the hair (`pHair`).
pub fn fn_006031e0(e: &mut Engine, this: Ptr<TESNPC>, hair: Ptr) {
    e.set(this, TESNPC::pHair, hair);
}

// Translated from 00603200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the eye colour (`pEyeColor`).
pub fn fn_00603200(e: &mut Engine, this: Ptr<TESNPC>, eyes: Ptr) {
    e.set(this, TESNPC::pEyeColor, eyes);
}

// Translated from 00603220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Comparison function for the list built by `fn_00603280`: both arguments
/// are pointers to form pointers; 0 when either is null, otherwise the
/// `strcmp` of the two forms' full names (`TESFullName` at `+0xd0`, whose
/// string getter `00408da0` returns the character pointer or the empty
/// string).
pub fn fn_00603220(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    if first.is_null() || second.is_null() {
        return 0;
    }
    let first_form = e.mem.u32(first.addr());
    let second_form = e.mem.u32(second.addr());
    if first_form == 0 || second_form == 0 {
        return 0;
    }
    let second_name = e
        .call(0x0040_8da0, &args![second_form + COMPONENT_FULL_NAME])
        .u32();
    let first_name = e
        .call(0x0040_8da0, &args![first_form + COMPONENT_FULL_NAME])
        .u32();
    e.call(STRCMP, &args![first_name, second_name]).i32()
}

// Translated from 00603280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects into the `BSSimpleArray` at `list` every NPC form of the data
/// handler's object list whose race is `race` and whose sex (`GetSex`) is
/// `sex`, has actor flag bit `4` set, and is not the player's base form
/// (`007cb2e0` appends); when that finds nothing, the search is repeated
/// once without the flag test. The result is sorted by name with
/// `fn_00603220` (`00729970`). Does nothing for a null list.
pub fn fn_00603280(e: &mut Engine, race: Ptr, sex: i32, list: Ptr) {
    if list.is_null() {
        return;
    }
    let mut require_flag = true;
    loop {
        let handler = e.global::<u32>(DATA_HANDLER);
        let objects = e.call(GET_WORD_AT_4, &args![handler]).u32();
        let mut node = e.call(GET_WORD_AT_4, &args![objects]).u32();
        while node != 0 {
            if e.call(GET_FORM_TYPE, &args![node]).u32() == 0x2a {
                let matches = e.call(NPC_GET_RACE, &args![node]).u32() == race.addr()
                    && e.call(GET_SEX, &args![node]).i32() == sex
                    && (!require_flag
                        || e.call(
                            TEST_ACTOR_FLAGS,
                            &args![node + COMPONENT_ACTOR_BASE_DATA, 4u32],
                        )
                        .bool());
                if matches {
                    let player = e.global::<u32>(PLAYER_SINGLETON);
                    let player_base = e.call(GET_WORD_AT_20, &args![player]).u32();
                    if node != player_base {
                        e.with_stack(4, |e, slot| {
                            e.mem.set_u32(slot.addr(), node);
                            e.call(0x007c_b2e0, &args![list, slot]);
                        });
                    }
                }
            }
            node = e.call(GET_WORD_AT_20, &args![node]).u32();
        }
        if e.call(0x0076_b610, &args![list]).bool() && require_flag {
            require_flag = false;
        } else {
            break;
        }
    }
    e.call(0x0072_9970, &args![list, 0x0060_3220u32]);
}

// Translated from 00603370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::InitItem` (Xbox PDB, virtual `0x88`): does nothing when the form
/// already has flag `8` (`004013e0`). Otherwise registers an NPC with actor
/// flag `4` in the preset list at `011cb764` (`005ae3d0`), runs the
/// `InitItem` of its script, container, AI, race, actor-base, spell,
/// touch-spell and destructible components (each with the NPC as argument),
/// resolves the class and combat style form ids to forms (`004839c0` after
/// `AddCompileIndex` `00485d50`, then `__RTDynamicCast`; the combat style
/// reports an error through `005b5e40` when it is missing), calls
/// `InitValues(false)` when the NPC is auto-calc, gives a default combat
/// style when it has none, sets `00565210(this, true)` when its health is 0,
/// computes the height from the race when it is `0.0`, and marks the form
/// with flag `8` (`00484ab0(true)`).
pub fn tesnpc_init_item(e: &mut Engine, this: Ptr<TESNPC>) {
    let base = this.addr();
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    if e.call(
        TEST_ACTOR_FLAGS,
        &args![base + COMPONENT_ACTOR_BASE_DATA, 4u32],
    )
    .bool()
    {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), base);
            e.call(0x005a_e3d0, &args![CHARGEN_PRESET_FACE_LIST, slot]);
        });
    }
    e.call(0x0048_cd90, &args![base + COMPONENT_SCRIPTABLE, this]);
    e.call(0x0048_1bb0, &args![base + COMPONENT_CONTAINER, this]);
    e.call(0x0047_f4c0, &args![base + COMPONENT_AI_FORM, this]);
    e.call(0x0048_b920, &args![base + COMPONENT_RACE, this]);
    e.call(0x0047_d8b0, &args![base + COMPONENT_ACTOR_BASE_DATA, this]);
    e.call(0x0048_d400, &args![base + COMPONENT_SPELL_LIST, this]);
    e.call(0x0047_beb0, &args![base + COMPONENT_TOUCH_SPELL, this]);
    e.call(0x0047_8be0, &args![base + COMPONENT_DESTRUCTIBLE, this]);

    if e.get(this, TESNPC::pCl) != Ptr::NULL {
        let class_id = e.get(this, TESNPC::pCl).addr();
        let (class, _) = resolve_form_id(e, this, class_id, TYPE_TES_CLASS);
        e.set(this, TESNPC::pCl, class);
    }

    if e.vcall(base, VSLOT_IS_AUTO_CALC, &args![]).bool() {
        tesnpc_init_values(e, this, false);
    }

    if e.get(this, TESNPC::pCombatStyle) != Ptr::NULL {
        let style_id = e.vcall(base, VSLOT_GET_COMBAT_STYLE, &args![]).u32();
        if style_id != 0 {
            let (style, resolved_id) = resolve_form_id(e, this, style_id, TYPE_TES_COMBAT_STYLE);
            e.set(this, TESNPC::pCombatStyle, style);
            if style.is_null() {
                let name = e.vcall(base, VSLOT_GET_FORM_NAME, &args![]).u32();
                let form_id = e.call(0x0084_e3a0, &args![this]).u32();
                e.call(
                    0x005b_5e40,
                    &args![0x0104_a600u32, resolved_id, form_id, name],
                );
            }
        }
    }

    if e.vcall(base, VSLOT_GET_COMBAT_STYLE, &args![]).u32() == 0 {
        let default_style = e.call(0x0050_5000, &args![]).u32();
        e.vcall(base, VSLOT_SET_COMBAT_STYLE, &args![default_style]);
    }

    let health = e.call(GET_WORD_AT_4, &args![base + COMPONENT_HEALTH]).u32();
    if health == 0 {
        e.call(0x0056_5210, &args![this, 1u32]);
    }

    let height = e.get(this, TESNPC::fHeight) as f64;
    let zero: f64 = e.global(ZERO_DOUBLE);
    if height == zero {
        let race_height = tesnpc_get_race_height(e, this);
        e.set(this, TESNPC::fHeight, race_height);
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// The `InitItem` pattern for a form id: records the file the form id
/// belongs to (`TESForm::GetFile(-1)`, then `AddCompileIndex` on the id),
/// looks the form up (`004839c0`) and casts it from `TESForm` to `target`.
fn resolve_form_id(e: &mut Engine, this: Ptr<TESNPC>, form_id: u32, target: u32) -> (Ptr, u32) {
    let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
    let (form, resolved) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), form_id);
        e.call(0x0048_5d50, &args![slot, file]);
        let resolved = e.mem.u32(slot.addr());
        (e.call(0x0048_39c0, &args![resolved]).u32(), resolved)
    });
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
        )
        .ptr();
    (cast, resolved)
}

// Translated from 006035e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::Copy` (virtual `0x108`): when `source` is an NPC (form type
/// `0x2a`), copies each component from it (`CopyComponent` of the AI, spell
/// list, touch spell, race, actor-base data, health, attributes, name,
/// model, script, container, animation and destructible components), then
/// the `0x1c` bytes of NPC data (`memcpy`), the class pointer, the combat
/// style (through virtual `0x188`/`0x18c`), the blood impact material
/// (virtual `0x58`/`0x5c` of the actor-base component), the head and face
/// (`fn_00603790`), and the height and weight.
pub fn tesnpc_copy(e: &mut Engine, this: Ptr<TESNPC>, source: Ptr<TESNPC>) {
    if e.call(GET_FORM_TYPE, &args![source]).u32() != 0x2a {
        return;
    }
    let base = this.addr();
    let from = source.addr();
    e.call(0x0047_f190, &args![base + COMPONENT_AI_FORM, source]);
    e.call(0x0048_d630, &args![base + COMPONENT_SPELL_LIST, source]);
    e.call(0x0047_bdd0, &args![base + COMPONENT_TOUCH_SPELL, source]);
    e.call(0x0048_b870, &args![base + COMPONENT_RACE, source]);
    e.call(
        0x0047_d220,
        &args![base + COMPONENT_ACTOR_BASE_DATA, source],
    );
    e.call(0x0048_7240, &args![base + COMPONENT_HEALTH, source]);
    e.call(0x0048_0000, &args![base + COMPONENT_ATTRIBUTES, source]);
    e.call(0x0048_70f0, &args![base + COMPONENT_FULL_NAME, source]);
    e.call(0x0048_9430, &args![base + COMPONENT_MODEL, source]);
    e.call(0x0048_cce0, &args![base + COMPONENT_SCRIPTABLE, source]);
    e.call(0x0048_1c80, &args![base + COMPONENT_CONTAINER, source]);
    e.call(0x0047_fb50, &args![base + COMPONENT_ANIMATION, source]);
    e.call(0x0047_86e0, &args![base + COMPONENT_DESTRUCTIBLE, source]);
    let source_data = e.call(NPC_GET_DATA, &args![source]).u32();
    e.call(MEMCPY, &args![base + 0x114, source_data, 0x1cu32]);
    let class = e.call(NPC_GET_CLASS, &args![source]).ptr();
    e.set(this, TESNPC::pCl, class);
    let combat_style = e.vcall(from, VSLOT_GET_COMBAT_STYLE, &args![]).u32();
    e.vcall(base, VSLOT_SET_COMBAT_STYLE, &args![combat_style]);
    let blood = e
        .vcall(
            from + COMPONENT_ACTOR_BASE_DATA,
            VSLOT_GET_BLOOD_IMPACT_MATERIAL,
            &args![],
        )
        .u32();
    e.vcall(
        base + COMPONENT_ACTOR_BASE_DATA,
        VSLOT_SET_BLOOD_IMPACT_MATERIAL,
        &args![blood],
    );
    fn_00603790(e, this, source);
    let height = e.get(source, TESNPC::fHeight);
    e.set(this, TESNPC::fHeight, height);
    let weight = e.get(source, TESNPC::fWeight);
    e.set(this, TESNPC::fWeight, weight);
}

// Translated from 00603790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the head of `source` into this NPC: the head-part list is emptied
/// (`00470470`) and refilled from the source's non-empty entries (`00905820`
/// appends), hair, hair length, eye colour and hair colour are copied (the
/// hair through `fn_00603b50`), the face-gen coordinate is copied from the
/// source's (`006529a0`), and the face NPC becomes `source` unless this NPC
/// is the player's base form.
pub fn fn_00603790(e: &mut Engine, this: Ptr<TESNPC>, source: Ptr<TESNPC>) {
    let own_parts = e.call(NPC_GET_HEAD_PARTS, &args![this]).u32();
    e.call(0x0047_0470, &args![own_parts]);
    let hair = fn_00603b50(e, source);
    e.set(this, TESNPC::pHair, hair);
    let hair_length = e.call(NPC_GET_HAIR_LENGTH, &args![source]).f32();
    e.set(this, TESNPC::fHairLength, hair_length);
    let eyes = e.call(NPC_GET_EYE_COLOR, &args![source]).ptr();
    e.set(this, TESNPC::pEyeColor, eyes);
    let hair_color = e.call(NPC_GET_HAIR_COLOR, &args![source]).u32();
    e.set(this, TESNPC::iHairColor, hair_color);
    let mut node = e.call(NPC_GET_HEAD_PARTS, &args![source]).u32();
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let item = e.call(0x0068_15c0, &args![node]).u32();
        let own_parts = e.call(NPC_GET_HEAD_PARTS, &args![this]).u32();
        e.call(0x0090_5820, &args![own_parts, item]);
        node = e.call(GET_WORD_AT_4, &args![node]).u32();
    }
    let source_coord = e.call(NPC_FACE_COORD_POINTER, &args![source]).u32();
    let own_coord = e.call(NPC_FACE_COORD_POINTER, &args![this]).u32();
    e.call(
        FACEGEN_COPY_COORD,
        &args![source_coord, own_coord, 0u32, 0u32],
    );
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if player == 0 || this.addr() != e.call(GET_WORD_AT_20, &args![player]).u32() {
        e.call(0x004c_0cd0, &args![this, source]);
    }
}

// Translated from 00603880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::Compare` (virtual `0x10c`): true when `other` (cast to `TESNPC`
/// with `__RTDynamicCast`) is not an NPC or differs from this one: in its
/// components (`TESForm::CompareAllComponents`, `00485270`), the `0x1c`
/// bytes of NPC data, class, hair, hair length, eye colour, hair colour,
/// face-gen coordinate (`00652900`), combat style, blood impact material,
/// head-part count and every head part of this NPC being in `other`'s list
/// (`005f65d0`), height and weight.
pub fn tesnpc_compare(e: &mut Engine, this: Ptr<TESNPC>, other: Ptr) -> bool {
    let cast = e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_TES_NPC, 0u32],
    );
    let other = cast.ptr::<TESNPC>();
    if other.is_null() {
        return true;
    }
    let base = this.addr();
    let theirs = other.addr();
    if e.call(0x0048_5270, &args![this, other]).bool() {
        return true;
    }
    let other_data = e.call(NPC_GET_DATA, &args![other]).u32();
    if e.call(MEMCMP, &args![base + 0x114, other_data, 0x1cu32])
        .i32()
        != 0
    {
        return true;
    }
    if e.get(this, TESNPC::pCl).addr() != e.call(NPC_GET_CLASS, &args![other]).u32() {
        return true;
    }
    if e.get(this, TESNPC::pHair) != fn_00603b50(e, other) {
        return true;
    }
    let other_length = e.call(NPC_GET_HAIR_LENGTH, &args![other]).f32();
    if e.get(this, TESNPC::fHairLength) != other_length {
        return true;
    }
    if e.get(this, TESNPC::pEyeColor).addr() != e.call(NPC_GET_EYE_COLOR, &args![other]).u32() {
        return true;
    }
    if e.get(this, TESNPC::iHairColor) != e.call(NPC_GET_HAIR_COLOR, &args![other]).u32() {
        return true;
    }
    let own_coord = e.call(NPC_FACE_COORD_POINTER, &args![this]).u32();
    let other_coord = e.call(NPC_FACE_COORD_POINTER, &args![other]).u32();
    if e.call(FACEGEN_COORDS_DIFFER, &args![own_coord, other_coord])
        .bool()
    {
        return true;
    }
    let own_style = e.vcall(base, VSLOT_GET_COMBAT_STYLE, &args![]).u32();
    let other_style = e.vcall(theirs, VSLOT_GET_COMBAT_STYLE, &args![]).u32();
    if own_style != other_style {
        return true;
    }
    let own_blood = e
        .vcall(
            base + COMPONENT_ACTOR_BASE_DATA,
            VSLOT_GET_BLOOD_IMPACT_MATERIAL,
            &args![],
        )
        .u32();
    let other_blood = e
        .vcall(
            theirs + COMPONENT_ACTOR_BASE_DATA,
            VSLOT_GET_BLOOD_IMPACT_MATERIAL,
            &args![],
        )
        .u32();
    if own_blood != other_blood {
        return true;
    }
    let own_parts = e.call(NPC_GET_HEAD_PARTS, &args![this]).u32();
    let own_count = e.call(0x005a_e380, &args![own_parts]).u32();
    let other_parts = e.call(NPC_GET_HEAD_PARTS, &args![other]).u32();
    let other_count = e.call(0x005a_e380, &args![other_parts]).u32();
    if own_count != other_count {
        return true;
    }
    let mut node = e.call(NPC_GET_HEAD_PARTS, &args![this]).u32();
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let item = e.call(0x0068_15c0, &args![node]).u32();
        let other_parts = e.call(NPC_GET_HEAD_PARTS, &args![other]).u32();
        if !e.call(0x005f_65d0, &args![other_parts, item]).bool() {
            return true;
        }
        node = e.call(GET_WORD_AT_4, &args![node]).u32();
    }
    if e.get(this, TESNPC::fHeight) != e.get(other, TESNPC::fHeight) {
        return true;
    }
    e.get(this, TESNPC::fWeight) != e.get(other, TESNPC::fWeight)
}

// Translated from 00603ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::GetFaceCoord` (Xbox PDB): fills the face-gen coordinate at
/// `coord`. It is initialised (`006521f0`); without a race it receives a copy
/// of the default coordinate (`006521e0`), otherwise the race's coordinate
/// for this NPC's sex (`005d9fb0`: race `+0x478` for male, `+0x3f8` for
/// female) is blended with the NPC's own (`00652af0`, no flag, scale
/// `0.0`).
pub fn tesnpc_get_face_coord(e: &mut Engine, this: Ptr<TESNPC>, coord: Ptr) {
    e.call(FACEGEN_INIT_COORD, &args![coord]);
    let race = e
        .call(GET_WORD_AT_4, &args![this.addr() + COMPONENT_RACE])
        .u32();
    if race == 0 {
        let default_coord = e.call(FACEGEN_DEFAULT_COORD, &args![]).u32();
        e.call(FACEGEN_COPY_COORD, &args![default_coord, coord, 0u32, 0u32]);
    } else {
        let own_coord = e.call(NPC_FACE_COORD_POINTER, &args![this]).u32();
        let sex = e.call(GET_SEX, &args![this]).u32();
        let race_coord = e.call(0x005d_9fb0, &args![race, sex]).u32();
        e.call(
            FACEGEN_BLEND_COORDS,
            &args![race_coord, own_coord, coord, 0u32, 0.0f32],
        );
    }
}

// Translated from 00603b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The NPC's hair: `pHair`, unless it has an original race and is not the
/// player's base form, in which case it is the race's default hair for the
/// NPC's sex (`00613870`, race `+0x94 + sex * 4`).
pub fn fn_00603b50(e: &mut Engine, this: Ptr<TESNPC>) -> Ptr {
    let mut hair = e.get(this, TESNPC::pHair);
    if e.get(this, TESNPC::pOriginalRace) != Ptr::NULL {
        let player = e.global::<u32>(PLAYER_SINGLETON);
        let player_base = e.call(GET_WORD_AT_20, &args![player]).u32();
        if this.addr() != player_base {
            let sex = e.call(GET_SEX, &args![this]).u32();
            let race = e
                .call(GET_WORD_AT_4, &args![this.addr() + COMPONENT_RACE])
                .u32();
            hair = e.call(0x0061_3870, &args![race, sex]).ptr();
        }
    }
    hair
}

// Translated from 00603bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the two head `NiPointer`s (`spHeadBiped` and `spHeadSkinned`).
pub fn fn_00603bb0(e: &mut Engine, this: Ptr<TESNPC>) {
    e.call(0x0066_b0d0, &args![this.addr() + 0x1c4, 0u32]);
    e.call(0x0066_b0d0, &args![this.addr() + 0x1c8, 0u32]);
}

// ---------------------------------------------------------------------------
// Derived values: skills, health, fatigue, height and weight

/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in
/// EAX. The value is passed as an `f64` argument (two words).
pub(crate) const FTOL: u32 = 0x00ec_62c0;
/// `SettingT` objects of the exe: `43d4d0(setting)` returns the address of an
/// integer value (`setting + 4`, or a scratch word for a null setting) and
/// `403e20(setting)` that of a float value.
const GET_INT_SETTING_VALUE: u32 = 0x0043_d4d0;
const GET_FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// `float` to nearest integer (`FISTP`, `00406d90`, cdecl).
const ROUND_FLOAT: u32 = 0x0040_6d90;
/// `TESAttributes::GetAttribute(index)` (`00480160`): byte `index - 1`.
const ATTRIBUTES_GET: u32 = 0x0048_0160;
/// `TESAttributes::SetAttributeValue(actorValue, value, flag)` (`00480180`).
const ATTRIBUTES_SET: u32 = 0x0048_0180;
/// `ActorValue::ToActorValue(kind, index)` (`0066ec10`, cdecl).
const TO_ACTOR_VALUE: u32 = 0x0066_ec10;
/// `TESActorBaseData::GetLevel` (`0047ded0`).
const GET_LEVEL: u32 = 0x0047_ded0;
/// `TESForm::GetFormID` folded accessor: the word at `+0xc`.
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// The form id of the player's base NPC form.
const PLAYER_BASE_FORM_ID: u32 = 7;
/// The `float` `5f1230` returns for an attribute component: byte 7 of the
/// attributes (`TESAttributes::GetAttribute(7)`).
const ATTRIBUTES_GET_SEVENTH: u32 = 0x005f_1230;

// Translated from 00603be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::InitValues` (Xbox PDB): for an auto-calc NPC (virtual `0x144`)
/// with a race and a class, computes the seven attributes (the class's
/// attribute bytes, at most 10) and the fourteen skills from them (the
/// derived skill formula `00643c20`, plus the class's share of the skill
/// points for its tag skills, at most 100), then the AI form's
/// `0047efb0` value (the class's `+0x58` word, or 0) and marks the form
/// changed (virtual `0x48`, flags `0x20c`). `extra_divisor` adds one to the
/// divisor (3) of the skill point budget.
///
/// The player's base NPC (form id 7) does not take attributes from the class
/// when the class's form id equals the integer setting at `011d0e7c`.
pub fn tesnpc_init_values(e: &mut Engine, this: Ptr<TESNPC>, extra_divisor: bool) {
    let base = this.addr();
    let mut value: f32 = 0.0;
    if e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32() == 0
        || e.get(this, TESNPC::pCl) == Ptr::NULL
    {
        return;
    }
    let mut is_player_base = false;
    if e.call(GET_FORM_ID, &args![this]).u32() == PLAYER_BASE_FORM_ID {
        is_player_base = true;
    }
    let mut use_class = true;
    if is_player_base {
        let setting = e.call(GET_INT_SETTING_VALUE, &args![0x011d_0e7cu32]).u32();
        let class = e.call(NPC_GET_CLASS, &args![this]).u32();
        let class_form_id = e.call(GET_FORM_ID, &args![class]).u32();
        if e.mem.u32(setting) == class_form_id {
            use_class = false;
        }
    }
    if !e.vcall(base, VSLOT_IS_AUTO_CALC, &args![]).bool() {
        return;
    }

    // The seven attributes.
    let ten_double: f64 = e.global(0x0102_0758);
    for i in 0..7u32 {
        let actor_value = e.call(TO_ACTOR_VALUE, &args![0u32, i]).u32();
        if use_class {
            let class = e.call(NPC_GET_CLASS, &args![this]).u32();
            let attribute = e.call(ATTRIBUTES_GET, &args![class + 0x38, i + 5]).u8();
            value = attribute as f32;
        }
        if value as f64 > ten_double {
            value = e.global(0x0101_7b78);
        }
        let rounded = e.call(ROUND_FLOAT, &args![value]).u32();
        e.call(
            ATTRIBUTES_SET,
            &args![base + COMPONENT_ATTRIBUTES, actor_value, rounded, 0u32],
        );
    }

    // The skill point budget.
    let luck_like = e
        .call(ATTRIBUTES_GET, &args![base + COMPONENT_ATTRIBUTES, 0xbu32])
        .u8();
    let attribute_9 = e
        .call(ATTRIBUTES_GET, &args![base + COMPONENT_ATTRIBUTES, 9u32])
        .u8();
    let per_level = e.call(0x0064_8c10, &args![attribute_9 as u32, 1u32]).i32();
    let level = e
        .call(GET_LEVEL, &args![base + COMPONENT_ACTOR_BASE_DATA])
        .u16() as i32;
    let budget = (level - 1).wrapping_mul(per_level);
    let mut divisor = extra_divisor as i32 + 3;
    let share = (budget as f64 / divisor as f64) as f32;
    let share = e.call(0x0040_4040, &args![share]).f32();
    let points_setting = e.call(GET_INT_SETTING_VALUE, &args![0x011c_d6c0u32]).u32();
    let points = e.mem.i32(points_setting);
    let class_share = (points as f64 * share as f64) as f32;
    let mut carry: f32 = 0.0;
    divisor -= 1;

    // The fourteen skills.
    let hundred_double: f64 = e.global(0x0101_7a40);
    for j in 0..14u32 {
        let actor_value = e.call(TO_ACTOR_VALUE, &args![2u32, j]).u32();
        if e.call(0x0040_6d70, &args![actor_value, 0x2000u32]).bool() {
            continue;
        }
        if !e.call(0x0047_f060, &args![actor_value]).bool() {
            continue;
        }
        let (index, from_table) = e.with_stack(8, |e, slot| {
            e.mem.set_i32(slot.addr(), -1);
            e.mem.set_f32(slot.addr() + 4, 0.0);
            e.call(0x0064_3bf0, &args![actor_value, slot, slot.addr() + 4]);
            (e.mem.i32(slot.addr()), e.mem.f32(slot.addr() + 4))
        });
        value = from_table;
        let stat = e.mem.u8(base.wrapping_add(index as u32).wrapping_add(0xb7));
        value = e
            .call(0x0064_3c20, &args![value, stat as f32, luck_like as f32])
            .f32();
        let class = e.get(this, TESNPC::pCl);
        if use_class && class != Ptr::NULL && e.call(0x005a_5f40, &args![class, actor_value]).bool()
        {
            let spent = class_share as f64 + carry as f64;
            let bonus_setting = e
                .call(GET_FLOAT_SETTING_VALUE, &args![0x011c_cf5cu32])
                .u32();
            let bonus = e.mem.f32(bonus_setting) as f64;
            value = (bonus + spent + value as f64) as f32;
            carry = 0.0;
            if value as f64 > hundred_double {
                carry = (value as f64 - hundred_double) as f32;
                if divisor != 0 {
                    carry = (carry as f64 / divisor as f64) as f32;
                }
            }
            divisor -= 1;
        }
        if value as f64 > hundred_double {
            value = e.global(0x0101_6410);
        }
        let skill = e.call(ROUND_FLOAT, &args![value]).u8();
        e.mem.set_u8(base + 0x114 + j, skill);
    }

    let class = e.get(this, TESNPC::pCl);
    if class != Ptr::NULL {
        if e.vcall(base + COMPONENT_ACTOR_BASE_DATA, 0x30, &args![])
            .bool()
        {
            let word = e.call(0x0062_86d0, &args![class]).u32();
            e.call(0x0047_efb0, &args![base + COMPONENT_AI_FORM, word, 0u32]);
        }
    } else {
        e.call(0x0047_efb0, &args![base + COMPONENT_AI_FORM, 0u32, 0u32]);
    }
    e.vcall(base, 0x48, &args![0x20cu32]);
}

// Translated from 00603f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::GetFormHealth` (Xbox PDB; `TESHealthForm` virtual `0x10`, so
/// `this` is the health component): the stored health (`00726070`), 0 when
/// it is negative.
pub fn tesnpc_get_form_health(e: &mut Engine, this: Ptr) -> i32 {
    let health = e.call(GET_WORD_AT_4, &args![this]).i32();
    health.max(0)
}

// Translated from 00603fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The NPC's hit points from its level (at least 1) and attributes: 0 when
/// the health component is 0, otherwise
/// `trunc(s3 * (level - 1) + (s2 * (s1 + attribute7) + base))` with the
/// float settings `s1` (`011cd27c`), `s2` (`011cd544`), `s3` (`011cd908`),
/// `base` the health component's value when `use_base` is set (else 0),
/// and `attribute7` the byte `005f1230` reads from the attributes; negative
/// results become 0.
pub fn fn_00603fc0(e: &mut Engine, this: Ptr<TESNPC>, use_base: bool) -> i32 {
    let base = this.addr();
    let mut level = e
        .call(GET_LEVEL, &args![base + COMPONENT_ACTOR_BASE_DATA])
        .u16() as i32;
    if level < 1 {
        level = 1;
    }
    let health = e.call(GET_WORD_AT_4, &args![base + COMPONENT_HEALTH]).u32();
    let result = if health != 0 {
        let start = if use_base { health } else { 0 };
        let start = start as f64;
        let attribute = e
            .call(ATTRIBUTES_GET_SEVENTH, &args![base + COMPONENT_ATTRIBUTES])
            .u8() as f64;
        let first = float_setting(e, 0x011c_d27c) as f64 + attribute;
        let second = float_setting(e, 0x011c_d544) as f64;
        let sum = second * first + start;
        let per_level = float_setting(e, 0x011c_d908) as f64;
        let total = per_level * (level - 1) as f64 + sum;
        e.call(FTOL, &args![total]).i32()
    } else {
        0
    };
    result.max(0)
}

/// The value of the float `SettingT` object at `setting` (`00403e20`).
fn float_setting(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(GET_FLOAT_SETTING_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

// Translated from 006040d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The NPC's fatigue from its level (at least 1) and attributes: 0 when the
/// `TESActorBaseData` fatigue (virtual `0x60`) is 0, otherwise
/// `trunc(s4 * level + (s2 * attribute7 + f))` with `f` the fatigue itself
/// when `use_fatigue` is set and the float setting `011cd974` otherwise,
/// `s2` the setting `011ce018` and `s4` the setting `011cdb54`; negative
/// results become 0.
pub fn fn_006040d0(e: &mut Engine, this: Ptr<TESNPC>, use_fatigue: bool) -> i32 {
    let base = this.addr();
    let mut level = e
        .call(GET_LEVEL, &args![base + COMPONENT_ACTOR_BASE_DATA])
        .u16() as i32;
    if level < 1 {
        level = 1;
    }
    let fatigue = e
        .vcall(base + COMPONENT_ACTOR_BASE_DATA, 0x60, &args![])
        .u16();
    let result = if fatigue != 0 {
        let start: f32 = if use_fatigue {
            let again = e
                .vcall(base + COMPONENT_ACTOR_BASE_DATA, 0x60, &args![])
                .u16();
            again as f32
        } else {
            float_setting(e, 0x011c_d974)
        };
        let attribute = e
            .call(ATTRIBUTES_GET_SEVENTH, &args![base + COMPONENT_ATTRIBUTES])
            .u8() as f64;
        let per_attribute = float_setting(e, 0x011c_e018) as f64;
        let sum = per_attribute * attribute + start as f64;
        let per_level = float_setting(e, 0x011c_db54) as f64;
        let total = per_level * level as f64 + sum;
        e.call(FTOL, &args![total]).i32()
    } else {
        0
    };
    result.max(0)
}

// Translated from 006041c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at the exe global `011c6210`.
pub fn fn_006041c0(e: &mut Engine) -> u32 {
    e.global(0x011c_6210)
}

// Translated from 006041d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::GetRaceHeight` (Xbox PDB): the race's height for this NPC's sex
/// (`00604210`), 0.0 without a race.
pub fn tesnpc_get_race_height(e: &mut Engine, this: Ptr<TESNPC>) -> f32 {
    let race = e.call(NPC_GET_RACE, &args![this]).u32();
    if race != 0 {
        let sex = e.call(GET_SEX, &args![this]).i32();
        fn_00604210(e, Ptr::new(race), sex)
    } else {
        0.0
    }
}

// Translated from 00604210 (decompiled, FalloutNV.exe 1.4.0.525)
/// A race's height for a sex: the float at `race + 0x60 + sex * 4` for sex 0
/// or 1, 0.0 for anything else.
pub fn fn_00604210(e: &mut Engine, race: Ptr, sex: i32) -> f32 {
    if (0..2).contains(&sex) {
        e.mem.f32(race.addr() + 0x60 + 4 * sex as u32)
    } else {
        0.0
    }
}

// Translated from 00604250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::GetWeight` (Xbox PDB): for the player's base form (form id 7)
/// the stored weight, or the race's weight for the NPC's sex (`006042d0`)
/// when the stored weight is 0.0 and the NPC has a race; for every other NPC
/// the exe returns the float at `+0x1f4` (`00944300`, the height field).
pub fn tesnpc_get_weight(e: &mut Engine, this: Ptr<TESNPC>) -> f32 {
    if e.call(GET_FORM_ID, &args![this]).u32() != PLAYER_BASE_FORM_ID {
        return e.call(0x0094_4300, &args![this]).f32();
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    let weight = e.get(this, TESNPC::fWeight);
    if weight as f64 == zero {
        let race = e.call(NPC_GET_RACE, &args![this]).u32();
        if race != 0 {
            let sex = e.call(GET_SEX, &args![this]).i32();
            return fn_006042d0(e, Ptr::new(race), sex);
        }
    }
    weight
}

// Translated from 006042d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A race's weight for a sex: the float at `race + 0x68 + sex * 4` for sex 0
/// or 1, 0.0 for anything else.
pub fn fn_006042d0(e: &mut Engine, race: Ptr, sex: i32) -> f32 {
    if (0..2).contains(&sex) {
        e.mem.f32(race.addr() + 0x68 + 4 * sex as u32)
    } else {
        0.0
    }
}

// ---------------------------------------------------------------------------
// Face textures, voice type, worn objects

/// `TESForm::GetOwnerMaster` (`00484ee0`): the first master file in the
/// form's source-file list, or null.
const GET_OWNER_MASTER: u32 = 0x0048_4ee0;
/// `sprintf_s(buffer, size, format, ...)` (`00406d00`, cdecl).
const SPRINTF_S: u32 = 0x0040_6d00;
/// The file name of a `TESFile` (`00891170`: `this + 0x20`).
const FILE_NAME: u32 = 0x0089_1170;
/// `_strnicmp(a, b, count)` (the wrapper `004564f0`, cdecl).
const STRNICMP: u32 = 0x0045_64f0;
/// `NiPointer` assignment from a raw pointer (`0066b0d0`) and from another
/// `NiPointer` (`006e5cc0`), the `NiPointer` constructor (`00633c90`) and
/// destructor (`0045cec0`).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_ASSIGN_POINTER: u32 = 0x006e_5cc0;
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;
/// `TESBitmask`-like setting reader: `00408d60(setting)` returns the address
/// of the boolean value of the setting object `setting` (or of a scratch
/// byte for null).
const GET_BOOL_SETTING_VALUE: u32 = 0x0040_8d60;
/// `"FalloutNV.ESM"`, `"update"` and the three file-name formats of the
/// face-mod textures.
const MASTER_FILE_NAME: u32 = 0x0104_a704;
const UPDATE_PREFIX: u32 = 0x0104_a714;
const FORMAT_FACE_MOD_ANY_RACE: u32 = 0x0104_a660;
const FORMAT_FACE_MOD_MALE: u32 = 0x0104_a694;
const FORMAT_FACE_MOD_FEMALE: u32 = 0x0104_a6cc;
/// The `BSFaceGenManager` singleton pointer (`011d59e8`); its `+0x119c` is
/// the `NiPointer` of the default base modulation texture.
const FACE_GEN_MANAGER: u32 = 0x011d_59e8;
/// The `TES` singleton pointer (`011dea10`).
const TES_SINGLETON: u32 = 0x011d_ea10;

// Translated from 00604310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::GetHeadPartModTextureFileName` (Xbox PDB): writes into `buffer`
/// (`size` bytes) the path of the face-mod texture number `index`:
/// `data\Textures\Characters\FaceMods\<plugin>\[F|M]<race id>_<npc id>_<index>.dds`
/// (the race id only for NPCs that can be of any race, with `F` or `M`
/// by sex; otherwise `<npc id>_<index>.dds`), ids reduced to their low 24
/// bits and the plugin the form's owner master (`FalloutNV.ESM` when that
/// file's name starts with `update`). An NPC with a face NPC defers to the
/// last NPC of its face-NPC chain. False when the form has no master.
pub fn tesnpc_get_head_part_mod_texture_file_name(
    e: &mut Engine,
    this: Ptr<TESNPC>,
    index: i32,
    buffer: Ptr,
    size: u32,
) -> bool {
    let face_npc = e.get(this, TESNPC::pFaceNPC);
    if face_npc != Ptr::NULL {
        let mut npc = face_npc.cast::<TESNPC>();
        while npc != Ptr::NULL {
            let next = fn_006044c0(e, npc);
            if next == Ptr::NULL {
                break;
            }
            npc = next;
        }
        return tesnpc_get_head_part_mod_texture_file_name(e, npc, index, buffer, size);
    }
    let file = e.call(GET_OWNER_MASTER, &args![this]).u32();
    if file == 0 {
        return false;
    }
    let form_id = e.call(GET_FORM_ID, &args![this]).u32() & 0x00ff_ffff;
    let race = e.call(NPC_GET_RACE, &args![this]).u32();
    let race_id = e.call(GET_FORM_ID, &args![race]).u32() & 0x00ff_ffff;
    let file_name = e.call(FILE_NAME, &args![file]).u32();
    let is_update = e
        .call(STRNICMP, &args![file_name, UPDATE_PREFIX, 6u32])
        .i32()
        == 0;
    let base = this.addr();
    let name = if is_update {
        MASTER_FILE_NAME
    } else {
        e.call(FILE_NAME, &args![file]).u32()
    };
    if e.vcall(base + COMPONENT_ACTOR_BASE_DATA, 0x2c, &args![])
        .bool()
    {
        let format = if e.call(GET_SEX, &args![this]).i32() == 1 {
            FORMAT_FACE_MOD_FEMALE
        } else {
            FORMAT_FACE_MOD_MALE
        };
        e.call(
            SPRINTF_S,
            &args![buffer, size, format, name, race_id, form_id, index],
        );
    } else {
        e.call(
            SPRINTF_S,
            &args![buffer, size, FORMAT_FACE_MOD_ANY_RACE, name, form_id, index],
        );
    }
    true
}

// Translated from 006044c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The face NPC (`pFaceNPC`).
pub fn fn_006044c0(e: &mut Engine, this: Ptr<TESNPC>) -> Ptr<TESNPC> {
    e.get(this, TESNPC::pFaceNPC).cast()
}

// Translated from 006044e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `NiPointer` at `slot` to the face-mod texture number `index` of
/// this NPC: null when the boolean setting `011d5adc` is set or the NPC is
/// the player's base form, the default base modulation texture for
/// `index >= 1`, otherwise the texture loaded (`TES::CreateTextureImage`,
/// `004568c0`) from the path `fn_00604310` builds, null when it fails.
pub fn fn_006044e0(e: &mut Engine, this: Ptr<TESNPC>, index: i32, slot: Ptr) {
    let setting = e.call(GET_BOOL_SETTING_VALUE, &args![0x011d_5adcu32]).u32();
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if e.mem.u8(setting) != 0 || this.addr() == e.call(GET_WORD_AT_20, &args![player]).u32() {
        e.call(NI_POINTER_ASSIGN, &args![slot, 0u32]);
        return;
    }
    if index >= 1 {
        let texture = bs_face_gen_manager_get_default_base_modulation_texture(e);
        e.call(NI_POINTER_ASSIGN, &args![slot, texture]);
        return;
    }
    e.with_stack(0x104, |e, buffer| {
        if !tesnpc_get_head_part_mod_texture_file_name(e, this, index, buffer, 0x104) {
            e.call(NI_POINTER_ASSIGN, &args![slot, 0u32]);
            return;
        }
        e.with_stack(4, |e, loaded| {
            e.call(NI_POINTER_CONSTRUCT, &args![loaded, 0u32]);
            let tes = e.global::<u32>(TES_SINGLETON);
            e.call(0x0045_68c0, &args![tes, buffer, loaded, 1u32, 0u32]);
            e.call(NI_POINTER_ASSIGN_POINTER, &args![slot, loaded]);
            e.call(NI_POINTER_DESTROY, &args![loaded]);
        });
    });
}

// Translated from 006045f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenManager::GetDefaultBaseModulationTexture` (Xbox PDB): the raw
/// pointer of the `NiPointer` at `+0x119c` of the face-gen manager, or of a
/// null `NiPointer` when there is no manager. (The `BSStringT` getter
/// `00559450` the exe uses is the folded "first word of `this`".)
pub fn bs_face_gen_manager_get_default_base_modulation_texture(e: &mut Engine) -> Ptr {
    let manager = e.global::<u32>(FACE_GEN_MANAGER);
    if manager != 0 {
        e.call(0x0055_9450, &args![manager + 0x119c]).ptr()
    } else {
        e.with_stack(4, |e, empty| {
            e.call(NI_POINTER_CONSTRUCT, &args![empty, 0u32]);
            let texture = e.call(0x0055_9450, &args![empty]).ptr();
            e.call(NI_POINTER_DESTROY, &args![empty]);
            texture
        })
    }
}

// Translated from 006046a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC`'s `TESActorBaseData::GetVoiceType` (virtual `0x68` of the
/// component, so `this` is the component, `+0x30` in the NPC): for the
/// player's base form it first stores the default voice type (default
/// object `0xe` or `0xf` for a male, `0x10` or `0x11` for a female, the
/// second of each pair when `006047a0` is set on the player) in
/// `pVoiceType`. Then returns `pVoiceType` when set, otherwise the race's
/// voice type for the NPC's sex (`00604780`), or null without a race.
pub fn tesnpc_get_voice_type(e: &mut Engine, this: Ptr) -> Ptr {
    let npc = Ptr::<TESNPC>::new(this.addr().wrapping_sub(COMPONENT_ACTOR_BASE_DATA));
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if npc.addr() == e.call(GET_WORD_AT_20, &args![player]).u32() {
        let alternate = fn_006047a0(e, Ptr::new(player)) != 0;
        let male = e.call(GET_SEX, &args![npc]).i32() == 0;
        let default_object = match (male, alternate) {
            (true, false) => 0xe_u32,
            (true, true) => 0xf,
            (false, false) => 0x10,
            (false, true) => 0x11,
        };
        let voice = e.call(0x0058_db10, &args![default_object]).u32();
        e.mem.set_u32(this.addr() + 0x20, voice);
    }
    let voice = e.mem.u32(this.addr() + 0x20);
    if voice != 0 {
        return Ptr::new(voice);
    }
    let race = e.call(GET_WORD_AT_4, &args![this.addr() + 0xdc]).u32();
    if race == 0 {
        return Ptr::NULL;
    }
    let sex = e.call(GET_SEX, &args![npc]).i32();
    fn_00604780(e, Ptr::new(race), sex)
}

// Translated from 00604780 (decompiled, FalloutNV.exe 1.4.0.525)
/// A race's voice type for a sex: the word at `race + 0x4fc + sex * 4`.
pub fn fn_00604780(e: &mut Engine, race: Ptr, sex: i32) -> Ptr {
    Ptr::new(
        e.mem.u32(
            race.addr()
                .wrapping_add(0x4fc)
                .wrapping_add((sex as u32).wrapping_mul(4)),
        ),
    )
}

// Translated from 006047a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x7c5` of the player character.
pub fn fn_006047a0(e: &mut Engine, player: Ptr) -> u8 {
    e.mem.u8(player.addr() + 0x7c5)
}

// ---------------------------------------------------------------------------
// Worn objects, templates, cloning

/// The `BSSimpleList`-like accessor pair of the data handler (`004ea950`:
/// `this + 0x60`, `006130e0`: `this + 0x80`).
const DATA_HANDLER_FIRST_LIST: u32 = 0x004e_a950;
const DATA_HANDLER_SECOND_LIST: u32 = 0x0061_30e0;
/// `TESActorBaseData::pTemplateForm` getter (`0059bb30`: `this + 0x24`, the
/// engine map names it `D3DTexture_LockRect`).
const GET_TEMPLATE_FORM: u32 = 0x0059_bb30;
/// `TESActorBaseData::UsesTemplateFlag(bit)` (`0047cd40`): bit `bit` of the
/// template-use flags (`+0x1a`); true for a bit outside `0..16`.
const USES_TEMPLATE_FLAG: u32 = 0x0047_cd40;
/// The destructor body the exe runs on the stack NPC (`006013c0`, no deallocation).
const TESNPC_DESTRUCT: u32 = 0x0060_13c0;
/// `TESNPC::SetClass` (`00601c70`) and the setters of the other components
/// (each stores a word at `+4` or similar; called by address).
const NPC_SET_CLASS: u32 = 0x0060_1c70;
const SET_WORD_AT_4: u32 = 0x006e_cd40;
/// The pointer to the game object at `011ddf38` (`004623f0(flag)` sets a
/// flag on it and returns the previous one).
const FLAG_OBJECT: u32 = 0x011d_df38;
const FLAG_OBJECT_SET: u32 = 0x0046_23f0;

/// `source ? source + offset : 0`: the compiler's null-checked address of a
/// component (`&source->component`).
fn component_of(source: u32, offset: u32) -> u32 {
    if source != 0 {
        source + offset
    } else {
        0
    }
}

// Translated from 006047c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Equips an actor with the worn objects of this NPC: with the data handler's
/// flag object (`004623f0`) switched off (unless `keep_flag`), removes the
/// actor's weapon (`00571b50`, when virtual `0x100` says it is an actor) and
/// everything worn (`004bfe50`), clears the process's saved acquire object
/// (virtual `0x168`, `0x16c`, `0x164` and `0x160` of the process at
/// `actor + 0x68`), then equips each of the 20 worn-object slots of the
/// NPC's default inventory (`004c8220`) not already covered by a body slot
/// (virtual `0x184` of the actor, `EquipObject`), and, when `equip_weapon`
/// is set and the actor does not refuse (virtual `0x22c`), the weapon of
/// the inventory (`004c7400`, equipped with `Actor::EquipObject` or put
/// into the process). The flag object is restored and permanent magic is
/// cast (`008c26e0`).
///
/// `_unused_1` and `_unused_2` are words the exe never reads.
pub fn fn_006047c0(
    e: &mut Engine,
    this: Ptr<TESNPC>,
    actor: Ptr,
    _unused_1: u32,
    equip_weapon: bool,
    _unused_2: u32,
    keep_flag: bool,
) {
    let mut saved_flag: u8 = 1;
    let flag_object = e.global::<u32>(FLAG_OBJECT);
    if !keep_flag {
        saved_flag = e.call(FLAG_OBJECT_SET, &args![flag_object, 0u32]).u8();
    }
    if e.vcall(actor.addr(), 0x100, &args![]).bool() {
        e.call(0x0057_1b50, &args![actor]);
    }
    let inventory = e.call(0x004b_f220, &args![actor]).u32();
    e.call(0x004b_fe50, &args![inventory, 1u32, actor]);

    if e.call(PROCESS_OF_ACTOR, &args![actor]).u32() != 0 {
        let process = e.call(PROCESS_OF_ACTOR, &args![actor]).u32();
        e.vcall(process, 0x168, &args![0u32]);
        let process = e.call(PROCESS_OF_ACTOR, &args![actor]).u32();
        e.vcall(process, 0x16c, &args![0u32]);
        let process = e.call(PROCESS_OF_ACTOR, &args![actor]).u32();
        e.vcall(process, 0x164, &args![0u32]);
        let process = e.call(PROCESS_OF_ACTOR, &args![actor]).u32();
        e.vcall(process, 0x160, &args![0u32, 0u32, 0u32]);
    }

    let mut biped_model: u32 = 0;
    for slot in 0..0x14u32 {
        let item = e
            .call(0x004c_8220, &args![inventory, this, slot, 1u32])
            .u32();
        if item == 0 {
            continue;
        }
        let mut equip = true;
        let form = e.call(0x0044_ddc0, &args![item]).u32();
        let covered = (biped_model != 0
            && e.call(FILLS_BIPED_SLOT, &args![biped_model, slot, 0u32, 0u32])
                .bool())
            || e.call(0x0057_5400, &args![actor, form]).bool();
        if !covered {
            let first_word = e.call(GET_FIRST_WORD, &args![item]).u32();
            let extra = if first_word != 0 {
                let held = e.call(GET_FIRST_WORD, &args![item]).u32();
                let itself = e.call(0x0068_15c0, &args![held]).u32();
                e.mem.u32(itself)
            } else {
                0
            };
            if slot == 2 {
                biped_model = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![form, 0u32, 0x0118_3108u32, 0x0118_3978u32, 0u32],
                    )
                    .u32();
            } else {
                let model = e.call(0x0048_0db0, &args![form]).u32();
                if e.call(FILLS_BIPED_SLOT, &args![model, 2u32, 0u32, 0u32])
                    .bool()
                {
                    equip = false;
                }
            }
            if equip {
                e.vcall(actor.addr(), 0x184, &args![form, 1u32, extra, 0u32]);
            }
        }
        e.call(0x0044_59e0, &args![item, 1u32]);
    }

    if equip_weapon && !e.vcall(actor.addr(), 0x22c, &args![0u32]).bool() {
        e.with_stack(4, |e, scale| {
            e.mem.set_f32(scale.addr(), 0.0);
            let weapon = e
                .call(0x004c_7400, &args![inventory, this, scale, 6u32, 1u32])
                .u32();
            if weapon == 0 {
                return;
            }
            let mut free_item = true;
            let weapon_form = e.call(0x0044_ddc0, &args![weapon]).u32();
            if !e.call(0x0057_5400, &args![actor, weapon_form]).bool() {
                let mut extra = 0;
                if e.call(GET_FIRST_WORD, &args![weapon]).u32() != 0 {
                    let held = e.call(GET_FIRST_WORD, &args![weapon]).u32();
                    let itself = e.call(0x0068_15c0, &args![held]).u32();
                    extra = e.mem.u32(itself);
                    free_item = false;
                }
                let count = e.call(GET_WORD_AT_4, &args![weapon]).u32();
                let equipped_form = e.call(0x0044_ddc0, &args![weapon]).u32();
                e.call(
                    0x0088_c830,
                    &args![actor, equipped_form, count, extra, 1u32, 0u32, 1u32],
                );
                if free_item {
                    e.call(0x0044_59e0, &args![weapon, 1u32]);
                }
            } else if e.call(PROCESS_OF_ACTOR, &args![actor]).u32() != 0 {
                let weapon_form = e.call(0x0044_ddc0, &args![weapon]).u32();
                if weapon_form != e.call(0x008a_1710, &args![actor]).u32() {
                    let process = e.call(PROCESS_OF_ACTOR, &args![actor]).u32();
                    let extra = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
                    e.vcall(process, 0x160, &args![weapon, extra, 0u32]);
                }
            }
        });
    }

    if !keep_flag {
        e.call(FLAG_OBJECT_SET, &args![flag_object, saved_flag as u32]);
    }
    if e.call(PROCESS_OF_ACTOR, &args![actor]).u32() != 0 {
        e.call(0x008c_26e0, &args![actor, 1u32]);
    }
}

/// `Actor::GetProcess` folded accessor (`008d8520`): the word at `+0x68`.
const PROCESS_OF_ACTOR: u32 = 0x008d_8520;
/// `TESBipedModelForm::FillsBipedSlot(slot, a, b)` (`00480af0`).
const FILLS_BIPED_SLOT: u32 = 0x0048_0af0;
/// The word at `+0` of `this` (`00559450`).
const GET_FIRST_WORD: u32 = 0x0055_9450;

// Translated from 00604ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The NPC's `TESActorBaseData::CopyFromTemplateForm` (virtual `0x10` of the
/// component, so `this` is the component, `+0x30` in the NPC; `template` is
/// an optional NPC form that overrides the stored template). Without either
/// nothing happens. Otherwise the source is the explicit template if it is
/// an NPC, else the stored template (unless `0047cdd0` says its form type is `0x2c` or `0x2d`, which the code treats as a
/// leveled form), else a default NPC built on the stack from the data
/// handler's default race and class. Then, for every template-use flag (bits
/// `9, 8, 7, 6, 0, 1, 2, 3, 4, 5` in that order), the matching component or
/// values are copied from the source: bit 9 script, 8 inventory, 7 name and
/// part of the actor flags, 6 animation/model/blood material/head/
/// destructible data, 0 voice, race, height, weight, class, karma, combat
/// style, 1 level, health-type values, attributes and skills, 2 factions,
/// 3 spells, 4 and 5 AI data and packages. An auto-calc NPC then recomputes
/// its values (`tesnpc_init_values`), and the template flags are rewritten
/// (`0047d1a0`, `0047cd90`) and `0047ccc0(true)` is called. The stack NPC is
/// destroyed (`006013c0`).
pub fn fn_00604ba0(e: &mut Engine, this: Ptr, template: Ptr) {
    let component = this.addr();
    let npc = Ptr::<TESNPC>::new(component.wrapping_sub(COMPONENT_ACTOR_BASE_DATA));
    let npc_addr = npc.addr();
    if e.call(GET_TEMPLATE_FORM, &args![this]).u32() == 0 && template.is_null() {
        return;
    }
    e.with_stack(TESNPC::SIZE, |e, default_npc| {
        tesnpc_tesnpc(e, default_npc.cast());
        let mut source: u32 = 0;
        if !e.call(0x0047_cdd0, &args![this]).bool() {
            source = e.call(GET_TEMPLATE_FORM, &args![this]).u32();
        }
        if !template.is_null() && e.call(GET_FORM_TYPE, &args![template]).u32() == 0x2a {
            source = template.addr();
        }
        if source == 0 {
            source = default_npc.addr();
            let handler = e.global::<u32>(DATA_HANDLER);
            let first = e.call(DATA_HANDLER_FIRST_LIST, &args![handler]).u32();
            let first = e.call(0x0068_15c0, &args![first]).u32();
            let race = e.mem.u32(first);
            e.call(SET_WORD_AT_4, &args![source + COMPONENT_RACE, race]);
            let second = e.call(DATA_HANDLER_SECOND_LIST, &args![handler]).u32();
            let second = e.call(0x0068_15c0, &args![second]).u32();
            let class = e.mem.u32(second);
            e.call(NPC_SET_CLASS, &args![source, class]);
        } else if e
            .call(
                GET_TEMPLATE_FORM,
                &args![source + COMPONENT_ACTOR_BASE_DATA],
            )
            .u32()
            != 0
            && !e
                .call(0x0047_cca0, &args![source + COMPONENT_ACTOR_BASE_DATA])
                .bool()
        {
            e.vcall(source + COMPONENT_ACTOR_BASE_DATA, 0x10, &args![0u32]);
        }
        let uses = |e: &mut Engine, bit: u32| e.call(USES_TEMPLATE_FLAG, &args![this, bit]).bool();
        let base_data = source + COMPONENT_ACTOR_BASE_DATA;

        if uses(e, 9) {
            e.call(
                0x0048_cce0,
                &args![component + 0xc4, component_of(source, COMPONENT_SCRIPTABLE)],
            );
        }
        if uses(e, 8) {
            e.call(
                0x0048_1c80,
                &args![component + 0x34, component_of(source, COMPONENT_CONTAINER)],
            );
        }
        if uses(e, 7) {
            e.call(
                0x0048_70f0,
                &args![component + 0xa0, component_of(source, COMPONENT_FULL_NAME)],
            );
            let own_flags = e.call(0x0071_7e50, &args![this]).u32();
            let own = e.mem.u32(own_flags) & 0xef9f_5df5;
            let their_flags = e.call(0x0071_7e50, &args![base_data]).u32();
            let theirs = e.mem.u32(their_flags) & 0x1060_a20a;
            e.call(0x0047_dd30, &args![this, own | theirs]);
        }
        if uses(e, 6) {
            e.call(
                0x0047_fb50,
                &args![component + 0x94, component_of(source, COMPONENT_ANIMATION)],
            );
            e.call(0x0050_eaa0, &args![npc, source]);
            let blood = e
                .vcall(base_data, VSLOT_GET_BLOOD_IMPACT_MATERIAL, &args![])
                .u32();
            e.vcall(component, VSLOT_SET_BLOOD_IMPACT_MATERIAL, &args![blood]);
            fn_00603790(e, npc, Ptr::new(source));
            e.call(
                0x0047_86e0,
                &args![
                    component + 0xd4,
                    component_of(source, COMPONENT_DESTRUCTIBLE)
                ],
            );
        }
        if uses(e, 0) {
            let value = e.call(0x0044_1110, &args![base_data]).u32();
            // 00604f00 and 00604f1d: `ECX` is the component itself (no `SUB ECX,0x30`): its
            // +0x1C and +0x20 (on the NPC these would be the object list's links).
            e.call(0x0050_f9a0, &args![this, value]);
            let voice = e.vcall(base_data, 0x68, &args![]).u32();
            e.call(0x0050_f9c0, &args![this, voice]);
            let flag = e.call(TEST_ACTOR_FLAGS, &args![base_data, 1u32]).u8();
            e.call(0x0047_dd50, &args![this, 1u32, flag as u32, 1u32]);
            let race = e.call(GET_WORD_AT_4, &args![source + COMPONENT_RACE]).u32();
            e.call(SET_WORD_AT_4, &args![component + 0xdc, race]);
            let height = e.call(0x0094_4300, &args![source]).f32();
            e.call(0x0094_42e0, &args![npc, height]);
            let weight = tesnpc_get_weight(e, Ptr::new(source));
            fn_006053f0(e, npc, weight);
            let class = e.call(NPC_GET_CLASS, &args![source]).u32();
            e.call(NPC_SET_CLASS, &args![npc, class]);
            let karma = e.vcall(base_data, 0x64, &args![]).f32();
            e.call(0x0047_e1c0, &args![this, karma]);
            let value = e.call(0x0047_d3d0, &args![this]).u16();
            e.call(0x0047_dea0, &args![this, value as u32]);
            let style = e.vcall(source, VSLOT_GET_COMBAT_STYLE, &args![]).u32();
            e.vcall(npc_addr, VSLOT_SET_COMBAT_STYLE, &args![style]);
        }
        if uses(e, 1) {
            let level = e.call(0x0047_d370, &args![base_data]).u16();
            e.call(0x0047_dfe0, &args![this, level as u32]);
            let flag = e.call(0x0046_1560, &args![base_data]).u8();
            e.call(0x005d_1070, &args![this, flag as u32]);
            let min = e.call(0x0047_d390, &args![base_data]).u16();
            e.call(0x0047_de40, &args![this, min as u32]);
            let max = e.call(0x0047_d3b0, &args![base_data]).u16();
            e.call(0x0047_de70, &args![this, max as u32]);
            e.call(
                0x0048_0000,
                &args![component + 0x88, component_of(source, COMPONENT_ATTRIBUTES)],
            );
            e.call(
                0x0048_7240,
                &args![component + 0x80, component_of(source, COMPONENT_HEALTH)],
            );
            let speed = e.call(0x008f_21d0, &args![base_data]).u16();
            e.call(0x0047_e270, &args![this, speed as u32]);
            let fatigue = e.vcall(base_data, 0x60, &args![]).u16();
            e.call(0x0047_e010, &args![this, fatigue as u32]);
            e.call(MEMCPY, &args![component + 0xe4, source + 0x114, 0xeu32]);
            e.call(MEMCPY, &args![component + 0xf2, source + 0x122, 0xeu32]);
            if e.call(0x0046_1560, &args![this]).bool() {
                e.vcall(npc_addr, 0x148, &args![1u32]);
            } else {
                let auto_calc = e.vcall(source, VSLOT_IS_AUTO_CALC, &args![]).u8();
                e.vcall(npc_addr, 0x148, &args![auto_calc as u32]);
            }
        }
        if uses(e, 2) {
            let from = component_of(source, COMPONENT_ACTOR_BASE_DATA);
            e.call(0x0047_ca20, &args![this, from]);
        }
        if uses(e, 3) {
            e.call(
                0x0048_d630,
                &args![component + 0x4c, component_of(source, COMPONENT_SPELL_LIST)],
            );
            e.call(
                0x0047_bdd0,
                &args![
                    component + 0x40,
                    component_of(source, COMPONENT_TOUCH_SPELL)
                ],
            );
        }
        if uses(e, 4) {
            e.call(
                0x0047_f1e0,
                &args![component + 0x60, component_of(source, COMPONENT_AI_FORM)],
            );
        }
        if uses(e, 5) {
            e.call(
                0x0047_f2d0,
                &args![component + 0x60, component_of(source, COMPONENT_AI_FORM)],
            );
        }
        if e.vcall(npc_addr, VSLOT_IS_AUTO_CALC, &args![]).bool() {
            tesnpc_init_values(e, npc, false);
        }
        let (rewritten, flags) = e.with_stack(4, |e, flags| {
            e.mem.set_u16(flags.addr(), 0);
            let template_form = e.call(0x0047_d1a0, &args![npc, flags]).u32();
            (template_form, e.mem.u16(flags.addr()))
        });
        if rewritten != 0 {
            e.call(0x0070_37c0, &args![this, rewritten]);
            e.call(0x0047_cd90, &args![this, flags as u32]);
        }
        e.call(0x0047_ccc0, &args![this, 1u32]);
        e.call(TESNPC_DESTRUCT, &args![default_npc]);
    });
}

// Translated from 006053f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the weight (`fWeight`).
pub fn fn_006053f0(e: &mut Engine, this: Ptr<TESNPC>, weight: f32) {
    e.set(this, TESNPC::fWeight, x87_quiet(weight));
}

// Translated from 00605410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC`'s virtual `0x178` (`Clone3D` in the Xbox PDB): for an NPC that
/// uses a template (`0047cdb0` on the actor-base data) and a reference that
/// is not `0056afc0`-excluded, takes the reference's factions
/// (`0047ce10`) and lets the reference's base form clone itself instead
/// (its virtual `0x178`). Otherwise clones the model (`TESBoundObject::
/// Clone3D`, `0050ee50`) and, when the model has a `Bip01` node (`004aae30`),
/// attaches a biped animation: the reference's existing one is rebuilt on
/// the model (`004aad00`), or a new `BipedAnim` (`0x2b4` bytes, `004aaca0`)
/// is created and given to the reference (virtual `0x1f0`); the model's
/// properties are updated (`00a5a040`, `00a59c60` with a default update
/// data built by `0043d410(0.0, 0, 0)`). Returns the model.
pub fn tesnpc_clone_3d(e: &mut Engine, this: Ptr<TESNPC>, reference: Ptr) -> Ptr {
    let base = this.addr();
    if e.call(0x0047_cdb0, &args![base + COMPONENT_ACTOR_BASE_DATA])
        .bool()
        && !reference.is_null()
        && !e.call(0x0056_afc0, &args![reference]).bool()
    {
        e.call(
            0x0047_ce10,
            &args![base + COMPONENT_ACTOR_BASE_DATA, reference],
        );
        let base_form = e.call(GET_WORD_AT_20, &args![reference]).u32();
        return e.vcall(base_form, 0x178, &args![reference]).ptr();
    }
    let mut model: u32 = 0;
    if !reference.is_null() {
        let mut biped = e.vcall(reference.addr(), 0x1e8, &args![]).u32();
        model = e.call(0x0050_ee50, &args![this, reference, 0u32]).u32();
        if biped != 0 {
            if e.call(0x004a_ae30, &args![model, ROOT_NODE_NAME]).u32() != 0 {
                e.call(0x004a_ad00, &args![biped, model]);
                update_model_properties(e, model);
            }
        } else if e.call(0x004a_ae30, &args![model, ROOT_NODE_NAME]).u32() != 0 {
            let memory = e.call(OPERATOR_NEW, &args![0x2b4u32]).u32();
            biped = if memory != 0 {
                e.call(0x004a_aca0, &args![memory, reference, model]).u32()
            } else {
                0
            };
            e.vcall(reference.addr(), 0x1f0, &args![biped]);
            update_model_properties(e, model);
        }
    }
    Ptr::new(model)
}

/// `"Bip01"`.
const ROOT_NODE_NAME: u32 = 0x0101_e460;
/// `operator new(size)` (`00401000`).
const OPERATOR_NEW: u32 = 0x0040_1000;

/// The tail both `Clone3D` paths share: `NiAVObject::UpdateProperties`
/// (`00a5a040`), then `NiAVObject::Update` (`00a59c60`) with an update-data
/// block built on the stack by `0043d410(0.0, false, false)`.
fn update_model_properties(e: &mut Engine, model: u32) {
    e.call(0x00a5_a040, &args![model]);
    e.with_stack(0x10, |e, data| {
        e.call(0x0043_d410, &args![data, 0.0f32, 0u32, 0u32]);
        e.call(0x00a5_9c60, &args![model, data]);
    });
}

// Translated from 006055d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `0050fbf0(this, argument)` and returns its result.
pub fn fn_006055d0(e: &mut Engine, this: Ptr<TESNPC>, argument: u32) -> u32 {
    e.call(0x0050_fbf0, &args![this, argument]).u32()
}

// Translated from 006055f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::CreateBipedAnim` (Xbox PDB): under a memory-context guard
/// (`00404eb0`, context `0x33`, this file, line `0x15d1`), if the reference
/// (`actor`) already has a biped animation (virtual `0x1e8`) logs
/// `"ANIMATION: This npc \"%s\" has already been used."` with the NPC's
/// name (virtual `0x130`, `005b5e40`) and returns the existing one;
/// otherwise creates a `BipedAnim` (`0x2b4` bytes, `004aaca0(reference,
/// 0)`), gives it to the reference (virtual `0x1f0`) and returns it.
pub fn tesnpc_create_biped_anim(e: &mut Engine, this: Ptr<TESNPC>, actor: Ptr) -> Ptr {
    e.with_stack(4, |e, guard| {
        e.call(
            0x0040_4eb0,
            &args![guard, 0x33u32, 1u32, 0x0104_a77cu32, 0x15d1u32],
        );
        let mut biped = e.vcall(actor.addr(), 0x1e8, &args![]).u32();
        if biped != 0 {
            let name = e.vcall(this.addr(), VSLOT_GET_FORM_NAME, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0104_a720u32, name]);
        } else {
            let memory = e.call(OPERATOR_NEW, &args![0x2b4u32]).u32();
            biped = if memory != 0 {
                e.call(0x004a_aca0, &args![memory, actor, 0u32]).u32()
            } else {
                0
            };
            e.vcall(actor.addr(), 0x1f0, &args![biped]);
        }
        e.call(0x0040_4ee0, &args![guard]);
        Ptr::new(biped)
    })
}

// ---------------------------------------------------------------------------
// Loaded head of an actor

/// Node names the head code searches for: `"UpperBody"`'s sibling table is
/// `00605d40`; `"Skin"` is the shader-property name prefix (`0104a7bc`) and
/// `0101f5f8` is the node name the skin search starts from.
const SKIN_PARENT_NODE_NAME: u32 = 0x0101_f5f8;
const SKIN_PROPERTY_PREFIX: u32 = 0x0104_a7bc;
/// The `float` `0.5`.
const HALF: u32 = 0x0101_6248;
/// `_strnicmp(a, b, count)` (CRT, cdecl).
const CRT_STRNICMP: u32 = 0x00ec_7ec0;
/// The `NiTArray`-like child lookups of a node: `0045bc00(index)` (the first
/// child), `00453470()` (the child count), `0043b4a0(index)` (a child).
const NODE_FIRST_CHILD: u32 = 0x0045_bc00;
const NODE_CHILD_COUNT: u32 = 0x0045_3470;
const NODE_CHILD_AT: u32 = 0x0043_b4a0;
/// `NiAVObject::GetProperty(type)` (`00a59d30`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// Casts a property to a shader property type (`00653270(type, object)`).
const CAST_TO_SHADER_PROPERTY: u32 = 0x0065_3270;
const SHADER_PROPERTY_TYPE: u32 = 0x011f_4a5c;
/// Looks a named node up below a root (`004aae30(root, name)`, cdecl).
const FIND_NODE_BY_NAME: u32 = 0x004a_ae30;
/// The face-gen parameter block built by `005dd590` and destroyed by
/// `005dd6f0` (`0xec` bytes; the lists are `BSSimpleArray`s of words).
const FACE_PARAMS_SIZE: u32 = 0xf0;
const FACE_PARAMS_CONSTRUCT: u32 = 0x005d_d590;
const FACE_PARAMS_DESTROY: u32 = 0x005d_d6f0;
/// Appends the word at the given address to the `BSSimpleArray` at `this`
/// (`0096a610`) or the `NiPointer` at that address to the one at `this`
/// (`0060ba10`).
const ARRAY_APPEND_WORD: u32 = 0x0096_a610;
const ARRAY_APPEND_NI_POINTER: u32 = 0x0060_ba10;
/// Fills a head node from the parameter block (`00655fa0(node, params)`).
const APPLY_FACE_PARAMS: u32 = 0x0065_5fa0;
/// The race's default hair/eye/head part tables by sex and index.
const RACE_HAIR_ENTRY: u32 = 0x0061_3970;
const RACE_HEAD_PART_ENTRY: u32 = 0x0061_3b20;
/// The race's list of eye colours (`00503650`: `this + 0xa8`) and the
/// emptiness test of a list node (`008256d0`).
const RACE_EYE_LIST: u32 = 0x0050_3650;
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_NODE_SELF: u32 = 0x0068_15c0;
/// `NiAVObject` helper (`004b3e60(object, node, scale, 1, 0)`, cdecl) that
/// attaches the object to the actor's head node; true on success.
const ATTACH_TO_HEAD: u32 = 0x004b_3e60;

// Translated from 006056f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the head of a loaded actor (`actor`; nothing when null or when
/// the setting read by `fn_00605d20` is false). Takes the actor's 3D
/// root (virtual `0x1d0`), stores the head node from virtual `0x1b4(0)` in
/// `spHeadSkinned`, finds the body node named by `fn_00605d40(0)`, and, when
/// the head node does not claim to be set up already (virtual `0x120`), walks
/// the children of the `0101f5f8` node's first child (or of the node
/// itself) for the first one whose property type 2 casts to a shader property
/// named `"Skin"` and which attaches (`004b3e60`) to the body; then it
/// tells the head node (virtual `0x124(true)`). Then, for an NPC with a race,
/// builds the face-gen parameter block (`005dd590`) on the stack: hair, hair
/// length, eyes (the race's first eye colour when the NPC has none), sex,
/// hair colour, the face coordinate (`fn_00603ad0`), for the eight head part
/// slots the race's hair entry, its head part entry and the slot name
/// (`fn_00605d40`), and, when `fn_00605d50` allows, the face-mod texture
/// (`fn_006044e0`); and applies it to both head nodes (`00655fa0`).
pub fn fn_006056f0(e: &mut Engine, this: Ptr<TESNPC>, actor: Ptr) {
    if actor.is_null() || fn_00605d20(e) == 0 {
        return;
    }
    let base = this.addr();
    let root = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
    let head_node = e.vcall(actor.addr(), 0x1b4, &args![0u32]).u32();
    let head_pointer = e
        .call(NI_POINTER_ASSIGN, &args![base + 0x1c8, head_node])
        .u32();
    let head = e.call(GET_FIRST_WORD, &args![head_pointer]).u32();
    let body_name = fn_00605d40(e, 0);
    let body = e.call(FIND_NODE_BY_NAME, &args![root, body_name]).u32();
    let body_node = if body != 0 {
        e.vcall(body, 0x1c, &args![]).u32()
    } else {
        0
    };
    if body_node == 0 || head == 0 {
        return;
    }

    if !e.vcall(head, 0x120, &args![]).bool() {
        let parent = e
            .call(FIND_NODE_BY_NAME, &args![root, SKIN_PARENT_NODE_NAME])
            .u32();
        if parent != 0 {
            let parent_node = e.vcall(parent, 0xc, &args![]).u32();
            if parent_node != 0 {
                let first = e.call(NODE_FIRST_CHILD, &args![parent_node, 0u32]).u32();
                if first != 0 {
                    let first_node = e.vcall(first, 0xc, &args![]).u32();
                    if first_node != 0 {
                        attach_skin_child(e, first_node, body_node, head);
                    }
                } else {
                    attach_skin_child(e, parent_node, body_node, head);
                }
            }
        }
    }

    let race = e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32();
    if race == 0 {
        return;
    }
    e.with_stack(FACE_PARAMS_SIZE, |e, params| {
        let block = params.addr();
        e.call(FACE_PARAMS_CONSTRUCT, &args![params]);
        let hair = fn_00603b50(e, this);
        e.mem.set_u32(block + 0x80, hair.addr());
        let hair_length = e.call(NPC_GET_HAIR_LENGTH, &args![this]).f32();
        e.mem.set_f32(block + 0x88, hair_length);
        let mut eyes = e.call(NPC_GET_EYE_COLOR, &args![this]).u32();
        e.mem.set_u32(block + 0x8c, eyes);
        let sex = e.call(GET_SEX, &args![this]).u32();
        e.mem.set_u32(block + 0x90, sex);
        let hair_color = e.call(NPC_GET_HAIR_COLOR, &args![this]).u32();
        e.mem.set_u32(block + 0x84, hair_color);
        tesnpc_get_face_coord(e, this, params);
        if eyes == 0 {
            let eye_list = e.call(RACE_EYE_LIST, &args![race]).u32();
            if eye_list != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![eye_list]).bool() {
                let item = e.call(LIST_NODE_SELF, &args![eye_list]).u32();
                eyes = e.mem.u32(item);
                e.mem.set_u32(block + 0x8c, eyes);
            }
        }
        for slot in 0..8u32 {
            let sex = e.call(GET_SEX, &args![this]).u32();
            let hair_entry = e.call(RACE_HAIR_ENTRY, &args![race, sex, slot]).u32();
            append_word(e, block + 0x94, hair_entry);
            let part_entry = e.call(RACE_HEAD_PART_ENTRY, &args![race, sex, slot]).u32();
            append_word(e, block + 0xa4, part_entry);
            let slot_name = fn_00605d40(e, slot);
            append_word(e, block + 0xb4, slot_name);
            if fn_00605d50(e) != 0 {
                e.with_stack(4, |e, texture| {
                    e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
                    fn_006044e0(e, this, slot as i32, texture);
                    e.call(ARRAY_APPEND_NI_POINTER, &args![block + 0xc4, texture]);
                    e.call(NI_POINTER_DESTROY, &args![texture]);
                });
            }
        }
        let texture_flag = fn_00605d50(e);
        e.mem.set_u8(block + 0xd4, texture_flag);
        let biped = e.call(GET_FIRST_WORD, &args![base + 0x1c4]).u32();
        e.call(APPLY_FACE_PARAMS, &args![biped, params]);
        let skinned = e.call(GET_FIRST_WORD, &args![base + 0x1c8]).u32();
        e.call(APPLY_FACE_PARAMS, &args![skinned, params]);
        e.call(FACE_PARAMS_DESTROY, &args![params]);
    });
}

/// Appends `value` to the `BSSimpleArray` of words at `array`.
fn append_word(e: &mut Engine, array: u32, value: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(ARRAY_APPEND_WORD, &args![array, slot]);
    });
}

/// The loop `fn_006056f0` runs over the children of `container`: the first
/// child that is a node, whose property of type 2 casts to a shader
/// property (`00653270`) whose name starts with `"Skin"` and which attaches
/// to the body node (`004b3e60(child, body, 0.5, 1, 0)`) makes the head
/// node set up (virtual `0x124(true)`) and ends the search.
fn attach_skin_child(e: &mut Engine, container: u32, body_node: u32, head: u32) {
    let count = e.call(NODE_CHILD_COUNT, &args![container]).u32();
    for index in 0..count {
        let child = e.call(NODE_CHILD_AT, &args![container, index]).u32();
        let object = if child != 0 {
            e.vcall(child, 0x1c, &args![]).u32()
        } else {
            0
        };
        if object == 0 {
            continue;
        }
        let property = e.call(GET_PROPERTY, &args![object, 2u32]).u32();
        let shader = e
            .call(
                CAST_TO_SHADER_PROPERTY,
                &args![SHADER_PROPERTY_TYPE, property],
            )
            .u32();
        if shader == 0 {
            continue;
        }
        let name = e.call(0x0041_3f40, &args![shader]).u32();
        let name = e.call(0x0043_b1b0, &args![name]).u32();
        if e.call(CRT_STRNICMP, &args![name, SKIN_PROPERTY_PREFIX, 4u32])
            .i32()
            != 0
        {
            continue;
        }
        let half: f32 = e.global(HALF);
        if e.call(ATTACH_TO_HEAD, &args![object, body_node, half, 1u32, 0u32])
            .bool()
        {
            e.vcall(head, 0x124, &args![1u32]);
            return;
        }
    }
}

// Translated from 00605d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The boolean setting at `011d5a30`.
pub fn fn_00605d20(e: &mut Engine) -> u8 {
    let value = e.call(GET_BOOL_SETTING_VALUE, &args![0x011d_5a30u32]).u32();
    e.mem.u8(value)
}

// Translated from 00605d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `index`-th word of the exe's table at `01199fc4` (the names of the
/// head part slots).
pub fn fn_00605d40(e: &mut Engine, index: u32) -> u32 {
    e.mem
        .u32(0x0119_9fc4u32.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00605d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The boolean setting at `011cb99c`.
pub fn fn_00605d50(e: &mut Engine) -> u8 {
    let value = e.call(GET_BOOL_SETTING_VALUE, &args![0x011c_b99cu32]).u32();
    e.mem.u8(value)
}

// ---------------------------------------------------------------------------
// Loading an NPC record

/// A record chunk tag as the exe compares it: the four characters as a
/// little-endian word.
const fn chunk_tag(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

const TAG_ACBS: u32 = chunk_tag(b"ACBS");
const TAG_AIDT: u32 = chunk_tag(b"AIDT");
const TAG_CNAM: u32 = chunk_tag(b"CNAM");
const TAG_CNTO: u32 = chunk_tag(b"CNTO");
const TAG_COED: u32 = chunk_tag(b"COED");
const TAG_DATA: u32 = chunk_tag(b"DATA");
const TAG_DEST: u32 = chunk_tag(b"DEST");
const TAG_DNAM: u32 = chunk_tag(b"DNAM");
const TAG_DSTD: u32 = chunk_tag(b"DSTD");
const TAG_EAMT: u32 = chunk_tag(b"EAMT");
const TAG_EDID: u32 = chunk_tag(b"EDID");
const TAG_EITM: u32 = chunk_tag(b"EITM");
const TAG_ENAM: u32 = chunk_tag(b"ENAM");
const TAG_FGGA: u32 = chunk_tag(b"FGGA");
const TAG_FGGS: u32 = chunk_tag(b"FGGS");
const TAG_FGTS: u32 = chunk_tag(b"FGTS");
const TAG_FNAM: u32 = chunk_tag(b"FNAM");
const TAG_FULL: u32 = chunk_tag(b"FULL");
const TAG_HCLR: u32 = chunk_tag(b"HCLR");
const TAG_HNAM: u32 = chunk_tag(b"HNAM");
const TAG_INAM: u32 = chunk_tag(b"INAM");
const TAG_KFFZ: u32 = chunk_tag(b"KFFZ");
const TAG_LNAM: u32 = chunk_tag(b"LNAM");
const TAG_MODL: u32 = chunk_tag(b"MODL");
const TAG_MODT: u32 = chunk_tag(b"MODT");
const TAG_NAM0: u32 = chunk_tag(b"NAM0");
const TAG_NAM1: u32 = chunk_tag(b"NAM1");
const TAG_NAM2: u32 = chunk_tag(b"NAM2");
const TAG_NAM3: u32 = chunk_tag(b"NAM3");
const TAG_NAM4: u32 = chunk_tag(b"NAM4");
const TAG_NAM6: u32 = chunk_tag(b"NAM6");
const TAG_NAM7: u32 = chunk_tag(b"NAM7");
const TAG_NAM9: u32 = chunk_tag(b"NAM9");
const TAG_OBND: u32 = chunk_tag(b"OBND");
const TAG_PKID: u32 = chunk_tag(b"PKID");
const TAG_PNAM: u32 = chunk_tag(b"PNAM");
const TAG_RNAM: u32 = chunk_tag(b"RNAM");
const TAG_SCRI: u32 = chunk_tag(b"SCRI");
const TAG_SNAM: u32 = chunk_tag(b"SNAM");
const TAG_SPLO: u32 = chunk_tag(b"SPLO");
const TAG_TPLT: u32 = chunk_tag(b"TPLT");
const TAG_VTCK: u32 = chunk_tag(b"VTCK");
const TAG_ZNAM: u32 = chunk_tag(b"ZNAM");

/// `TESFile` methods (`this` is the file): the form type of the record being
/// read, the current chunk tag, the chunk size, whether the chunk's data
/// needs byte-swapping, advancing to the next chunk (true when there is
/// one), the file's name (`this + 0x20`) and the file offset of the record.
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
const FILE_GET_CHUNK_TAG: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
const FILE_RECORD_OFFSET: u32 = 0x0046_7bb0;
/// `TESFile::GetChunkData(destination)` (`004727f0`, a word),
/// `GetChunkData_ov2` (`00472840`, a 16-bit value) and `GetChunkData_ov3`
/// (`00472890(destination, size)`).
const FILE_GET_CHUNK_WORD: u32 = 0x0047_27f0;
const FILE_GET_CHUNK_U16: u32 = 0x0047_2840;
const FILE_GET_CHUNK_BYTES: u32 = 0x0047_2890;
/// The file's format version (`00403570`, a 16-bit value).
const FILE_VERSION: u32 = 0x0040_3570;
/// `memset(destination, value, size)` (`00403d30`, cdecl).
const MEMSET: u32 = 0x0040_3d30;
/// `TESForm::LoadForm(file)` (`00485110`), `TESForm::LoadData(file,
/// destination, size)` (`004861f0`) and the expected size of the data chunk
/// (`00486620`).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
const FORM_LOAD_DATA: u32 = 0x0048_61f0;
const FORM_DATA_HEADER_SIZE: u32 = 0x0048_6620;
/// `TESForm::AddCompileIndex(&formId, file)` (`00485d50`, cdecl) and the
/// lookup by form id (`004839c0`).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The log function with a format string (`005b5e40`, cdecl).
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// Byte-swap helpers called when the chunk needs swapping: a float or 32-bit
/// word (`00401080(address, 0)`), a 16-bit value (`00407a90(address, 0)`),
/// and the structures (`00462230` container entry, `00503210` faction entry,
/// `00503240` AI data, `0047cb90` actor-base data).
const SWAP_WORD: u32 = 0x0040_1080;
const SWAP_U16: u32 = 0x0040_7a90;
const SWAP_CONTAINER_ENTRY: u32 = 0x0046_2230;
const SWAP_FACTION_ENTRY: u32 = 0x0050_3210;
const SWAP_AI_DATA: u32 = 0x0050_3240;
const SWAP_ACTOR_BASE_DATA: u32 = 0x0047_cb90;
/// The container item `CNTO` created and `COED` is waiting to extend
/// (exe global `011cb7d8`).
const CONTAINER_ITEM_BEING_LOADED: u32 = 0x011c_b7d8;
/// `TESContainer::AddObject(entry)` (`004817f0`) returns the new item.
const CONTAINER_ADD_ENTRY: u32 = 0x0048_17f0;
/// `ContainerItemExtra::ContainerItemExtra` (`0040e690`) and the loader of
/// its chunk (`0040e780`).
const CONTAINER_EXTRA_CONSTRUCT: u32 = 0x0040_e690;
const CONTAINER_EXTRA_LOAD: u32 = 0x0040_e780;
/// The chunk loaders of the components: `BGSDestructibleObjectForm::
/// LoadChunk(component, file)`, `TESFullName` (`00487050`), `TESModel::
/// LoadModelChunk` (`004892d0`), `TESAnimation` (`0047faa0`).
const DESTRUCTIBLE_LOAD_CHUNK: u32 = 0x0047_81e0;
const FULL_NAME_LOAD: u32 = 0x0048_7050;
const MODEL_LOAD_CHUNK: u32 = 0x0048_92d0;
const ANIMATION_LOAD: u32 = 0x0047_faa0;
/// Other component setters: `TESAIForm` package (`0047f500`) and AI data
/// (`0047f0b0`), `TESSpellList::AddSpell` (`0048d290`), touch-spell
/// animation (`00483170`), `TESActorBaseData::SetFactionRank(id, rank)`
/// (`0047d800`), the script form's `InitItem` (`0048cd90`).
const AI_FORM_ADD_PACKAGE: u32 = 0x0047_f500;
const AI_FORM_SET_DATA: u32 = 0x0047_f0b0;
const SPELL_LIST_ADD_SPELL: u32 = 0x0048_d290;
const TOUCH_SPELL_SET_ANIMATION: u32 = 0x0048_3170;
const SET_FACTION_RANK: u32 = 0x0047_d800;
const SCRIPTABLE_INIT_ITEM: u32 = 0x0048_cd90;
/// `TESNPC::SetHairLength` (`00601c10(float)`).
const NPC_SET_HAIR_LENGTH: u32 = 0x0060_1c10;
/// `std::vector<float>` helpers of a face-gen matrix: resize (`0060b340`),
/// the iterator at an index (`0060b370(&iterator, index)`) and its
/// dereference (`00629ab0`, a word on the stack).
const MATRIX_RESIZE: u32 = 0x0060_b340;
const MATRIX_ITERATOR_AT: u32 = 0x0060_b370;
const ITERATOR_ELEMENT: u32 = 0x0062_9ab0;
/// `TESObjectREFR::SetObjectReference(baseForm)` (`00575690`).
const REFERENCE_SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
/// `TESActorBaseData::GetSpeedMultiplier` (`008f21d0`, a 16-bit value).
const GET_SPEED_MULTIPLIER: u32 = 0x008f_21d0;
/// The `TESForm` virtual slots `Load` calls: `LoadObjectBound(file)`
/// (`0xe0`) and the editor id setter (`0x134`, `const char *`).
const VSLOT_LOAD_OBJECT_BOUND: u32 = 0xe0;
const VSLOT_SET_EDITOR_ID: u32 = 0x134;

/// The `__RTDynamicCast` type descriptors of the forms `Load` casts to.
const TYPE_TES_EYES: u32 = 0x0118_6388;
const TYPE_TES_HAIR: u32 = 0x0118_63a0;
const TYPE_BGS_HEAD_PART: u32 = 0x0118_63b8;

/// The messages `Load` logs (exe strings).
const MESSAGE_INVALID_DATA_FORMAT: u32 = 0x0104_a5d0;
const MESSAGE_FACE_TEXTURE_FOUND: u32 = 0x0104_a4d8;
const MESSAGE_NO_EYES: u32 = 0x0104_a518;
const MESSAGE_NO_HAIR: u32 = 0x0104_a554;
const MESSAGE_NO_HEAD_PART: u32 = 0x0104_a590;
const MESSAGE_SPEED_ZERO: u32 = 0x0104_a498;

// Translated from 00602190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::Load` (Xbox PDB, virtual `0x20`): reads an NPC record from
/// `file`. False without further reading when the file's current record is
/// not an NPC (form type `0x2a`). Otherwise it stores the record's file
/// offset, loads the common form header (`TESForm::LoadForm`), clears form
/// flag `8`, and handles each chunk until there are no more: `EDID`
/// (editor id, up to `0x200` bytes), `OBND`, `FULL`, `MODL`/`MODT`,
/// `SCRI`, `CNTO`/`COED` (container items and their extras, the item under
/// construction kept in the global `011cb7d8`), `ACBS` (`0x18` bytes into
/// the actor-base data; the template-use flags are cleared for versions
/// before 8), `SNAM` (factions), `SPLO`, `EITM`, `EAMT`, `AIDT` (`0x14`
/// bytes, converted for versions before 4, 6 and 7), `PKID`, `VTCK`, `INAM`,
/// `TPLT`, `RNAM`, `KFFZ`, `DEST`/`DSTD`, `DATA` (`0x1c` bytes of NPC data,
/// or a shorter old format), `CNAM`, `HNAM`, `ENAM`, `PNAM` (forms looked
/// up by id and cast), `LNAM` (hair length clamped to `0..=1`), `HCLR`/
/// `NAM9`, `NAM4`, `NAM6`/`NAM7` (height and weight), `FNAM`, `ZNAM`, and the
/// face-gen coordinates `FGGS`, `FGGA`, `FGTS` (floats into the matrix of
/// the race coordinate at `[0][0]`, `[0][1]` and `[1][0]`); `NAM0` to
/// `NAM3` only log that face textures were found (once). Afterwards the
/// player's base form (form id 7) is set as the player reference's object
/// when that has none, and a speed multiplier of zero is logged. Returns
/// true.
pub fn tesnpc_load(e: &mut Engine, this: Ptr<TESNPC>, file: Ptr) -> bool {
    let base = this.addr();
    let file_addr = file.addr();
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u8() != 0x2a {
        return false;
    }
    let record_offset = e.call(FILE_RECORD_OFFSET, &args![file]).u32();
    e.set(this, TESNPC::iFileOffset, record_offset);
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    let mut logged_face_texture = false;
    loop {
        let tag = e.call(FILE_GET_CHUNK_TAG, &args![file]).u32();
        if tag == 0 {
            break;
        }
        match tag {
            TAG_CNAM => {
                let class = chunk_word(e, file_addr);
                e.set(this, TESNPC::pCl, Ptr::new(class));
            }
            TAG_DATA => load_data_chunk(e, this, file),
            TAG_NAM4 => {
                let material = chunk_word(e, file_addr);
                e.set(this, TESNPC::eBloodImpactMaterial, material);
            }
            TAG_NAM0 | TAG_NAM1 | TAG_NAM2 | TAG_NAM3 => {
                if !logged_face_texture {
                    let file_name = e.call(FILE_NAME, &args![file]).u32();
                    let form_id = e.call(GET_FORM_ID, &args![this]).u32();
                    let name = e.vcall(base, VSLOT_GET_FORM_NAME, &args![]).u32();
                    e.call(
                        LOG_MESSAGE,
                        &args![MESSAGE_FACE_TEXTURE_FOUND, name, form_id, file_name],
                    );
                    logged_face_texture = true;
                }
            }
            TAG_NAM9 | TAG_HCLR => {
                e.call(FILE_GET_CHUNK_WORD, &args![file, base + 0x1d8]);
            }
            TAG_NAM6 => {
                e.call(FILE_GET_CHUNK_WORD, &args![file, base + 0x1f4]);
            }
            TAG_NAM7 => {
                e.call(FILE_GET_CHUNK_WORD, &args![file, base + 0x1f8]);
            }
            TAG_FGGS | TAG_FGGA | TAG_FGTS => {
                let (matrix_row, matrix_column) = match tag {
                    TAG_FGGS => (0, 0),
                    TAG_FGGA => (0, 1),
                    _ => (1, 0),
                };
                load_face_gen_coordinate(e, this, file, matrix_row, matrix_column);
            }
            TAG_DSTD | TAG_DEST => {
                let component = component_of(base, COMPONENT_DESTRUCTIBLE);
                e.call(DESTRUCTIBLE_LOAD_CHUNK, &args![component, file]);
            }
            TAG_PKID => {
                let package = chunk_word(e, file_addr);
                e.call(
                    AI_FORM_ADD_PACKAGE,
                    &args![base + COMPONENT_AI_FORM, package],
                );
            }
            TAG_COED | TAG_CNTO => load_container_chunk(e, this, file),
            TAG_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size.max(1), |e, text| {
                    e.call(FILE_GET_CHUNK_BYTES, &args![file, text, 0x200u32]);
                    e.vcall(base, VSLOT_SET_EDITOR_ID, &args![text]);
                });
            }
            TAG_OBND => {
                e.vcall(base, VSLOT_LOAD_OBJECT_BOUND, &args![file]);
            }
            TAG_FULL => {
                let component = component_of(base, COMPONENT_FULL_NAME);
                e.call(FULL_NAME_LOAD, &args![component, file]);
            }
            TAG_MODL | TAG_MODT => {
                let component = component_of(base, COMPONENT_MODEL);
                e.call(MODEL_LOAD_CHUNK, &args![component, file]);
            }
            TAG_SCRI => {
                let script = chunk_word(e, file_addr);
                e.call(SET_WORD_AT_4, &args![base + COMPONENT_SCRIPTABLE, script]);
                e.call(
                    SCRIPTABLE_INIT_ITEM,
                    &args![base + COMPONENT_SCRIPTABLE, this],
                );
            }
            TAG_VTCK => {
                let voice = chunk_word(e, file_addr);
                e.mem.set_u32(base + 0x50, voice);
            }
            TAG_EITM => {
                let spell = chunk_word(e, file_addr);
                e.call(SET_WORD_AT_4, &args![base + COMPONENT_TOUCH_SPELL, spell]);
            }
            TAG_AIDT => load_ai_data_chunk(e, this, file),
            TAG_ACBS => {
                e.call(FILE_GET_CHUNK_BYTES, &args![file, base + 0x34, 0x18u32]);
                if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                    e.call(SWAP_ACTOR_BASE_DATA, &args![base + 0x34]);
                }
                if e.call(FILE_VERSION, &args![file]).u16() < 8 {
                    e.mem.set_u16(base + 0x4a, 0);
                }
                e.mem.set_u16(base + 0x3a, 0);
            }
            TAG_SPLO => {
                let spell = chunk_word(e, file_addr);
                e.call(
                    SPELL_LIST_ADD_SPELL,
                    &args![base + COMPONENT_SPELL_LIST, spell],
                );
            }
            TAG_EAMT => {
                let animation = chunk_u16(e, file_addr);
                e.call(
                    TOUCH_SPELL_SET_ANIMATION,
                    &args![base + COMPONENT_TOUCH_SPELL, animation as u32],
                );
            }
            TAG_TPLT => {
                let template = chunk_word(e, file_addr);
                e.mem.set_u32(base + 0x54, template);
            }
            TAG_INAM => {
                let item = chunk_word(e, file_addr);
                e.mem.set_u32(base + 0x4c, item);
            }
            TAG_SNAM => {
                e.with_stack(8, |e, entry| {
                    e.call(MEMSET, &args![entry, 0u32, 8u32]);
                    e.call(FILE_GET_CHUNK_BYTES, &args![file, entry, 8u32]);
                    if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                        e.call(SWAP_FACTION_ENTRY, &args![entry]);
                    }
                    let faction = e.mem.u32(entry.addr());
                    let rank = e.mem.u8(entry.addr() + 4);
                    e.call(
                        SET_FACTION_RANK,
                        &args![base + COMPONENT_ACTOR_BASE_DATA, faction, rank as u32],
                    );
                });
            }
            TAG_RNAM => {
                let race = chunk_word(e, file_addr);
                e.call(SET_WORD_AT_4, &args![base + COMPONENT_RACE, race]);
            }
            TAG_KFFZ => {
                let component = component_of(base, COMPONENT_ANIMATION);
                e.call(
                    ANIMATION_LOAD,
                    &args![base + COMPONENT_ANIMATION, component, file],
                );
            }
            TAG_DNAM => {
                e.call(FILE_GET_CHUNK_BYTES, &args![file, base + 0x114, 0x1cu32]);
            }
            TAG_ENAM => {
                let (eyes, id) = lookup_chunk_form(e, file, TYPE_TES_EYES);
                if eyes.is_null() {
                    log_missing_form(e, this, MESSAGE_NO_EYES, id);
                } else {
                    fn_00603200(e, this, eyes);
                }
            }
            TAG_FNAM => {
                if e.call(FILE_CHUNK_SIZE, &args![file]).u32() >= 2 {
                    e.call(FILE_GET_CHUNK_U16, &args![file, base + 0x1d0]);
                    if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                        e.call(SWAP_U16, &args![base + 0x1d0, 0u32]);
                    }
                }
            }
            TAG_HNAM => {
                let (hair, id) = lookup_chunk_form(e, file, TYPE_TES_HAIR);
                if hair.is_null() {
                    log_missing_form(e, this, MESSAGE_NO_HAIR, id);
                } else {
                    fn_006031e0(e, this, hair);
                }
            }
            TAG_LNAM => {
                let length = e.with_stack(4, |e, slot| {
                    e.call(FILE_GET_CHUNK_WORD, &args![file, slot]);
                    e.mem.f32(slot.addr())
                });
                let zero: f64 = e.global(ZERO_DOUBLE);
                let one: f64 = e.global(ONE_DOUBLE);
                let mut length = length;
                if (length as f64) < zero {
                    length = 0.0;
                }
                if (length as f64) > one {
                    length = 1.0;
                }
                e.call(NPC_SET_HAIR_LENGTH, &args![this, length]);
            }
            TAG_PNAM => {
                let (part, id) = lookup_chunk_form(e, file, TYPE_BGS_HEAD_PART);
                if part.is_null() {
                    log_missing_form(e, this, MESSAGE_NO_HEAD_PART, id);
                } else {
                    let list = e.call(NPC_GET_HEAD_PARTS, &args![this]).u32();
                    e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), part.addr());
                        e.call(0x005a_e3d0, &args![list, slot]);
                    });
                }
            }
            TAG_ZNAM => {
                let style = chunk_word(e, file_addr);
                e.vcall(base, VSLOT_SET_COMBAT_STYLE, &args![style]);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }

    if e.call(GET_FORM_ID, &args![this]).u32() == PLAYER_BASE_FORM_ID {
        let player = e.global::<u32>(PLAYER_SINGLETON);
        if player != 0 && e.call(GET_WORD_AT_20, &args![player]).u32() == 0 {
            e.call(REFERENCE_SET_OBJECT_REFERENCE, &args![player, this]);
        }
    }
    if e.call(
        GET_SPEED_MULTIPLIER,
        &args![base + COMPONENT_ACTOR_BASE_DATA],
    )
    .u16()
        == 0
    {
        let form_id = e.call(GET_FORM_ID, &args![this]).u32();
        let name = e.vcall(base, VSLOT_GET_FORM_NAME, &args![]).u32();
        e.call(LOG_MESSAGE, &args![MESSAGE_SPEED_ZERO, name, form_id]);
    }
    true
}

/// The `double` `1.0`.
const ONE_DOUBLE: u32 = 0x0101_2070;

/// Reads the current chunk as one word (`TESFile::GetChunkData`).
fn chunk_word(e: &mut Engine, file: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.call(FILE_GET_CHUNK_WORD, &args![file, slot]);
        e.mem.u32(slot.addr())
    })
}

/// Reads the current chunk as a 16-bit value (`GetChunkData_ov2`).
fn chunk_u16(e: &mut Engine, file: u32) -> u16 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u16(slot.addr(), 0);
        e.call(FILE_GET_CHUNK_U16, &args![file, slot]);
        e.mem.u16(slot.addr())
    })
}

/// The chunk handlers `HNAM`, `ENAM` and `PNAM` share: read the form id,
/// fix it up for the plugin load order (`AddCompileIndex`), look the form
/// up and cast it from `TESForm` to `target`. Returns the form (null when it
/// does not exist or has another type) and the fixed-up id.
fn lookup_chunk_form(e: &mut Engine, file: Ptr, target: u32) -> (Ptr, u32) {
    let (form, id) = e.with_stack(4, |e, slot| {
        e.call(FILE_GET_CHUNK_WORD, &args![file, slot]);
        e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
        let id = e.mem.u32(slot.addr());
        (e.call(LOOKUP_FORM, &args![id]).u32(), id)
    });
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
        )
        .ptr();
    (cast, id)
}

/// Logs that a form named by a chunk was not found: `format` takes the form
/// id, the NPC name and the NPC form id.
fn log_missing_form(e: &mut Engine, this: Ptr<TESNPC>, format: u32, id: u32) {
    let form_id = e.call(GET_FORM_ID, &args![this]).u32();
    let name = e.vcall(this.addr(), VSLOT_GET_FORM_NAME, &args![]).u32();
    e.call(LOG_MESSAGE, &args![format, id, name, form_id]);
}

/// The `DATA` chunk: its size beyond the form's own header (`00486620`) is
/// the size of the NPC data. Nothing beyond the header means no NPC data
/// (only the header is loaded); `0x1c` or `0xe` (half the size) is accepted
/// silently, anything else is logged; then the data is loaded into the
/// `NPC_DATA` at `+0x114`.
fn load_data_chunk(e: &mut Engine, this: Ptr<TESNPC>, file: Ptr) {
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    let header = e.call(FORM_DATA_HEADER_SIZE, &args![this]).u32();
    let data_size = size.wrapping_sub(header);
    if data_size == 0 {
        e.call(FORM_LOAD_DATA, &args![this, file, 0u32, 0u32]);
        return;
    }
    if data_size != 0x1c && data_size.wrapping_shl(1) != 0x1c {
        let form_id = e.call(GET_FORM_ID, &args![this]).u32();
        let name = e.vcall(this.addr(), VSLOT_GET_FORM_NAME, &args![]).u32();
        e.call(
            LOG_MESSAGE,
            &args![MESSAGE_INVALID_DATA_FORMAT, name, form_id],
        );
    }
    e.call(
        FORM_LOAD_DATA,
        &args![this, file, this.addr() + 0x114, data_size & 0xffff],
    );
}

/// A face-gen coordinate chunk: `size / 4` floats read into the matrix
/// `[row][column]` of the race coordinate (resized first), byte-swapped
/// when the file needs it.
fn load_face_gen_coordinate(e: &mut Engine, this: Ptr<TESNPC>, file: Ptr, row: u32, column: u32) {
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    let count = size >> 2;
    let matrix = this.addr() + 0x134 + row * 0x40 + column * 0x20;
    e.call(MATRIX_RESIZE, &args![matrix, count, 1u32]);
    e.with_stack(size.max(1), |e, buffer| {
        let data = buffer.addr();
        e.call(MEMSET, &args![buffer, 0u32, size]);
        e.call(FILE_GET_CHUNK_BYTES, &args![file, buffer, size]);
        for index in 0..count {
            if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                e.call(SWAP_WORD, &args![data + index * 4, 0u32]);
            }
            let element = e.with_stack(0x10, |e, iterator| {
                let at = e
                    .call(MATRIX_ITERATOR_AT, &args![matrix, iterator, index])
                    .u32();
                e.call(ITERATOR_ELEMENT, &args![at, 0u32]).u32()
            });
            let value = e.mem.f32(data + index * 4);
            e.mem.set_f32(element, value);
        }
    });
}

/// The `CNTO` and `COED` chunks: a `CNTO` (8 bytes: item and count) adds a
/// container entry and remembers it in the global `011cb7d8`; the `COED`
/// that follows creates the entry's extra data (`0xc` bytes) and loads it
/// from the chunk, then forgets the entry.
fn load_container_chunk(e: &mut Engine, this: Ptr<TESNPC>, file: Ptr) {
    let chunk = e.call(FILE_GET_CHUNK_TAG, &args![file]).u32();
    if chunk == TAG_COED {
        let entry = e.global::<u32>(CONTAINER_ITEM_BEING_LOADED);
        if entry != 0 {
            let memory = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
            let extra = if memory != 0 {
                e.call(CONTAINER_EXTRA_CONSTRUCT, &args![memory]).u32()
            } else {
                0
            };
            e.mem.set_u32(entry + 8, extra);
            e.call(CONTAINER_EXTRA_LOAD, &args![extra, file]);
            e.set_global(CONTAINER_ITEM_BEING_LOADED, 0u32);
        }
    } else if chunk == TAG_CNTO {
        e.with_stack(8, |e, buffer| {
            e.call(MEMSET, &args![buffer, 0u32, 8u32]);
            e.call(FILE_GET_CHUNK_BYTES, &args![file, buffer, 8u32]);
            if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                e.call(SWAP_CONTAINER_ENTRY, &args![buffer]);
            }
            let item = e
                .call(
                    CONTAINER_ADD_ENTRY,
                    &args![this.addr() + COMPONENT_CONTAINER, buffer],
                )
                .u32();
            e.set_global(CONTAINER_ITEM_BEING_LOADED, item);
        });
    }
}

/// The `AIDT` chunk: `0x14` bytes of AI data (byte-swapped when the file
/// needs it). Files of version below 4 get their first byte raised by one
/// when it is above 1; below 6 the first byte is split into itself and the
/// byte at `+0xe`; below 7 the second byte is mirrored (`4 - value`).
fn load_ai_data_chunk(e: &mut Engine, this: Ptr<TESNPC>, file: Ptr) {
    e.with_stack(0x14, |e, buffer| {
        let data = buffer.addr();
        e.call(MEMSET, &args![buffer, 0u32, 0x14u32]);
        e.call(FILE_GET_CHUNK_BYTES, &args![file, buffer, 0x14u32]);
        if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
            e.call(SWAP_AI_DATA, &args![buffer]);
        }
        if e.call(FILE_VERSION, &args![file]).u16() < 4 && e.mem.u8(data) > 1 {
            let first = e.mem.u8(data).wrapping_add(1);
            e.mem.set_u8(data, first);
        }
        if e.call(FILE_VERSION, &args![file]).u16() < 6 {
            match e.mem.u8(data) {
                0 => e.mem.set_u8(data + 0xe, 0),
                1 => {
                    e.mem.set_u8(data, 0);
                    e.mem.set_u8(data + 0xe, 1);
                }
                2 => {
                    e.mem.set_u8(data, 1);
                    e.mem.set_u8(data + 0xe, 1);
                }
                3 => {
                    e.mem.set_u8(data, 1);
                    e.mem.set_u8(data + 0xe, 2);
                }
                4 => {
                    e.mem.set_u8(data, 2);
                    e.mem.set_u8(data + 0xe, 2);
                }
                5 => {
                    e.mem.set_u8(data, 3);
                    e.mem.set_u8(data + 0xe, 0);
                }
                _ => {}
            }
        }
        if e.call(FILE_VERSION, &args![file]).u16() < 7 {
            let second = 4u8.wrapping_sub(e.mem.u8(data + 1));
            e.mem.set_u8(data + 1, second);
        }
        e.call(
            AI_FORM_SET_DATA,
            &args![this.addr() + COMPONENT_AI_FORM, buffer],
        );
    });
}

// ---------------------------------------------------------------------------
// Session 2 (b0016): from `ReplaceRefModel` (00605d70) to the end of the
// block at 0060b1f0. The functions that read or write the change-flag
// buffers of the save/load code share the helper below.

/// `TESObjectREFR` (RTTI type descriptor), the source type of the actor casts.
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// `Actor` (RTTI type descriptor).
const TYPE_ACTOR: u32 = 0x0118_46d4;
/// `TESRace` (RTTI type descriptor).
const TYPE_TES_RACE: u32 = 0x0118_6370;
/// `TESPackage` and `DialoguePackage` (RTTI type descriptors).
const TYPE_TES_PACKAGE: u32 = 0x0118_46a0;
const TYPE_DIALOGUE_PACKAGE: u32 = 0x0119_9c3c;
/// `TESBoundObject` (RTTI type descriptor).
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;

/// `MiddleHighProcess::GetSavedAcquireObject`-style getter (`008d8520`): the
/// process of an actor.
fn process_of(e: &mut Engine, actor: u32) -> u32 {
    e.call(PROCESS_OF_ACTOR, &args![actor]).u32()
}

/// `NiAVObject::UpdateProperties`-less update: the update-data block built by
/// `0043d410(0.0, false, false)` and `NiAVObject::Update` (`00a59c60`) with it.
fn update_node(e: &mut Engine, node: u32) {
    e.with_stack(0x10, |e, data| {
        e.call(0x0043_d410, &args![data, 0.0f32, 0u32, 0u32]);
        e.call(0x00a5_9c60, &args![node, data]);
    });
}

/// `UpdateProperties` (`00a5a040`) and the virtual `0xbc` of a node, then
/// the update of [`update_node`].
fn refresh_node(e: &mut Engine, node: u32) {
    e.call(0x00a5_a040, &args![node]);
    e.vcall(node, 0xbc, &args![]);
    update_node(e, node);
}

/// The "is this change flag set" test of the save/load code:
/// `getter(buffer, scratch)` (`00428110`, or `0042ce30`) returns a flag
/// object and `004280f0(object, mask)` says whether `mask` is in it.
fn change_flag_set(e: &mut Engine, getter: u32, buffer: Ptr, mask: u32) -> bool {
    e.with_stack(4, |e, scratch| {
        let flags = e.call(getter, &args![buffer, scratch]).u32();
        e.call(0x0042_80f0, &args![flags, mask]).bool()
    })
}

/// `LoadFormID` (`008648a0`) on a load buffer, the form lookup (`004839c0`)
/// and a dynamic cast of the form to `target`.
fn load_form_cast(e: &mut Engine, buffer: Ptr, target: u32) -> u32 {
    let id = e.call(0x0086_48a0, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

/// The three-word position the virtual `0x1f4` of an actor returns a pointer
/// to, passed by value (with a trailing `0`) to `008bb520(target, x, y, z, 0)`.
fn face_actor_towards(e: &mut Engine, actor: u32, target: u32) {
    let position = e.vcall(actor, 0x1f4, &args![]).u32();
    let (x, y, z) = (
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    );
    e.call(0x008b_b520, &args![target, x, y, z, 0u32]);
}

// Translated from 00605d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::ReplaceRefModel` (Xbox PDB): reloads the biped parts of an
/// actor's model (`actor`). The body runs once, or twice for the player (the
/// second round with the first-person biped and node): it rebuilds the parts
/// (`fn_00606540`), loads them (`004ac1e0(biped, 0)`) and updates the node.
pub fn tesnpc_replace_ref_model(e: &mut Engine, this: Ptr<TESNPC>, actor: Ptr) {
    let mut biped = e.vcall(actor.addr(), 0x1e8, &args![]).u32();
    let mut node = e.call(0x0043_fcd0, &args![actor]).u32();
    let mut passes = 1u32;
    if actor.addr() == e.global::<u32>(PLAYER_SINGLETON) {
        passes = 2;
    }
    while passes != 0 {
        let player = e.global::<u32>(PLAYER_SINGLETON);
        if actor.addr() == player && passes == 1 {
            let first_person = e.call(0x004e_af60, &args![player]).u8() as u32;
            biped = e.call(0x0095_0b00, &args![player, first_person]).u32();
            node = e.call(0x0095_0bb0, &args![player, 1u32]).u32();
        }
        fn_00606540(e, this, actor, Ptr::new(biped), 0);
        e.call(0x004a_c1e0, &args![biped, 0u32]);
        if node != 0 {
            refresh_node(e, node);
        }
        passes -= 1;
    }
}

// Translated from 00605e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Equips the biped (`biped`) of an actor (`actor`) with this NPC's worn items
/// (`worn`, a list: count `0044ddc0`, element address `006a7ad0`). The race and
/// sex go to the biped first (`004ab250(race, female)`), then each item is put on
/// with `tesnpc_init_worn_object`, then `fn_006062e0` runs and the biped's part
/// loader `004ac1e0(biped, 1)` is called. The actor's process (`008d8520`) gets its
/// virtual `0x468(1)` before the items and `0x470` after them. Last, the 3D root
/// the actor's virtual `0x1d0` gives is updated.
///
pub fn fn_00605e70(e: &mut Engine, this: Ptr<TESNPC>, actor: Ptr, biped: Ptr, worn: Ptr) {
    let female = e.get(this, TESNPC::iActorBaseFlags) & 1 != 0;
    let race = e
        .call(GET_WORD_AT_4, &args![this.addr() + COMPONENT_RACE])
        .u32();
    e.call(0x004a_b250, &args![biped, race, female]);
    if !actor.is_null() && process_of(e, actor.addr()) != 0 {
        let process = process_of(e, actor.addr());
        e.vcall(process, 0x468, &args![1u32]);
    }
    let mut index = 0u32;
    while index < e.call(0x0044_ddc0, &args![worn]).u32() {
        let slot = e.call(0x006a_7ad0, &args![worn, index]).u32();
        let item = e.mem.u32(slot);
        tesnpc_init_worn_object(e, this, actor, biped, Ptr::new(item));
        index += 1;
    }
    fn_006062e0(e, this, actor, Ptr::NULL);
    if !actor.is_null() && process_of(e, actor.addr()) != 0 {
        let process = process_of(e, actor.addr());
        e.vcall(process, 0x470, &args![]);
    }
    e.call(0x004a_c1e0, &args![biped, 1u32]);
    let node = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
    if node != 0 {
        refresh_node(e, node);
    }
}

// Translated from 00605fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::BuildObjectArray` (Xbox PDB): for each of the 20 biped slots of
/// `slots` (`0043f220`) whose part (the slot's first word) answers true to
/// its virtual `0xe4`, appends the part to the `BSSimpleArray` `array`
/// (`007cb2e0`) unless `009962f0` finds it there already.
///
/// `_unused_1` is a word the exe never reads.
pub fn tesnpc_build_object_array(
    e: &mut Engine,
    _this: Ptr<TESNPC>,
    _unused_1: u32,
    slots: Ptr,
    array: Ptr,
) {
    for index in 0..0x14u32 {
        let slot = e.call(0x0043_f220, &args![slots, index]).u32();
        if e.mem.u32(slot) == 0 {
            continue;
        }
        let part = e.mem.u32(slot);
        if !e.vcall(part, 0xe4, &args![]).bool() {
            continue;
        }
        let part = e.mem.u32(slot);
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), part);
            if !e.call(0x0099_62f0, &args![array, cell]).bool() {
                e.call(0x007c_b2e0, &args![array, cell]);
            }
        });
    }
}

// Translated from 00606050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::InitWorn` (Xbox PDB): walks the biped slots `0..=0x15` (slots 3
/// and 5 are visited as `0x14` and `0x15`, and the loop skips them in
/// between), asks the actor's inventory changes (`004bf220`) for the item
/// worn in each (`004c8c10`) and equips it on the biped with
/// `tesnpc_init_worn_object`; an item that cannot be handled is logged.
pub fn tesnpc_init_worn(e: &mut Engine, this: Ptr<TESNPC>, actor: Ptr, biped: Ptr) {
    let changes = e.call(0x004b_f220, &args![actor]).u32();
    // The result of this cast is not used.
    e.call(
        RT_DYNAMIC_CAST,
        &args![actor, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
    );
    if actor.is_null() || biped.is_null() {
        return;
    }
    let mut slot: i32 = 0;
    while slot <= 0x15 {
        if slot == 3 || slot == 5 {
            slot += 1;
        } else if slot == 0x14 {
            slot = 3;
        } else if slot == 0x15 {
            slot = 5;
        }
        let entry = e.call(0x004c_8c10, &args![changes, slot, 0u32]).u32();
        if entry != 0 {
            let item = e.call(0x0044_ddc0, &args![entry]).u32();
            if entry != 0 {
                e.call(0x0044_59e0, &args![entry, 1u32]);
            }
            if item != 0 && !tesnpc_init_worn_object(e, this, actor, biped, Ptr::new(item)) {
                let name = e.call(0x0048_2720, &args![item]).u32();
                let slot_name = e.global::<u32>(0x0118_8b98 + (slot as u32).wrapping_mul(4));
                e.call(LOG_MESSAGE, &args![0x0104_a7c8u32, slot_name, name]);
            }
        }
        if slot == 3 {
            slot = 0x14;
        } else if slot == 5 {
            slot = 0x15;
        }
        slot += 1;
    }
}

// Translated from 006061b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::InitWornObject` (Xbox PDB): puts one worn item on the biped. A
/// form of type `0x28` (a weapon) goes to `004ab400(biped, item, 0)`; any
/// other item with a biped model (`00480db0`) is added to the biped with
/// this NPC's sex (`00480bd0(biped, female, -1)`), and one without is
/// logged. A biped model list (`00475020`) then has each of its models
/// added the same way. Always true.
///
/// `_unused_1` is a word the exe never reads.
pub fn tesnpc_init_worn_object(
    e: &mut Engine,
    this: Ptr<TESNPC>,
    _unused_1: Ptr,
    biped: Ptr,
    item: Ptr,
) -> bool {
    let female = e.get(this, TESNPC::iActorBaseFlags) & 1 != 0;
    let model = e.call(0x0048_0db0, &args![item]).u32();
    if e.call(GET_FORM_TYPE, &args![item]).u32() == 0x28 {
        e.call(0x004a_b400, &args![biped, item, 0u32]);
    } else if model != 0 {
        e.call(0x0048_0bd0, &args![model, biped, female, u32::MAX]);
    } else {
        let name = e.call(0x0048_2720, &args![item]).u32();
        e.call(LOG_MESSAGE, &args![0x0104_a828u32, name]);
    }
    let list = e.call(0x0047_5020, &args![item]).u32();
    if list != 0 && e.call(GET_WORD_AT_4, &args![list]).u32() != 0 {
        let first = e.call(GET_WORD_AT_4, &args![list]).u32();
        let mut node = e.call(0x0050_0940, &args![first]).u32();
        while node != 0 {
            if e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                return true;
            }
            let cell = e.call(LIST_NODE_SELF, &args![node]).u32();
            let form = e.mem.u32(cell);
            let model = e.call(0x0048_0db0, &args![form]).u32();
            if model != 0 {
                e.call(0x0048_0bd0, &args![model, biped, female, u32::MAX]);
            }
            node = e.call(GET_WORD_AT_4, &args![node]).u32();
        }
    }
    true
}

// Translated from 006062e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes a flag to the `BSFaceGenNiNodeBiped` and `BSFaceGenNiNodeSkinned`
/// children of a model root (`00450f90(node, flag)`) and to the head and part
/// nodes below it. `node` is the root (the actor's, `0043fcd0`, when null).
/// If the actor has an item in biped slot 0 (`004c8c10` on its inventory changes)
/// both face-gen nodes get 1 and the item is released (`004459e0`); otherwise they
/// get 0 and the head node (named by `fn_00605d40(1)`) and the two nodes whose
/// names `00657820` makes get a flag that is 1 when slot 1 holds an item (the
/// second part's flag is 0 when slot 10 holds one, 1 otherwise).
///
/// `_this` is the `ECX` word the exe never reads.
///
pub fn fn_006062e0(e: &mut Engine, _this: Ptr<TESNPC>, actor: Ptr, node: Ptr) {
    let mut node = node.addr();
    if node == 0 {
        node = e.call(0x0043_fcd0, &args![actor]).u32();
    }
    if node == 0 {
        return;
    }
    let biped_node = e
        .call(FIND_NODE_BY_NAME, &args![node, 0x0102_0408u32])
        .u32();
    let skinned_node = e
        .call(FIND_NODE_BY_NAME, &args![node, 0x0102_03f0u32])
        .u32();
    if biped_node == 0 || skinned_node == 0 {
        return;
    }
    let changes = e.call(0x004b_f220, &args![actor]).u32();
    let worn = e.call(0x004c_8c10, &args![changes, 0u32, 0u32]).u32();
    if worn != 0 {
        e.call(0x0045_0f90, &args![biped_node, 1u32]);
        e.call(0x0045_0f90, &args![skinned_node, 1u32]);
        e.call(0x0044_59e0, &args![worn, 1u32]);
        return;
    }
    e.call(0x0045_0f90, &args![biped_node, 0u32]);
    e.call(0x0045_0f90, &args![skinned_node, 0u32]);
    let head_name = fn_00605d40(e, 1);
    let head_node = e.call(FIND_NODE_BY_NAME, &args![node, head_name]).u32();
    let mut part_nodes = [0u32; 2];
    e.with_stack(0x20, |e, buffer| {
        for (index, part) in part_nodes.iter_mut().enumerate() {
            let name = e.call(0x0065_7820, &args![buffer, index as u32]).u32();
            *part = e.call(FIND_NODE_BY_NAME, &args![node, name]).u32();
        }
    });
    let mut hide_head = false;
    let mut hide_second = true;
    let slot_one = e.call(0x004c_8c10, &args![changes, 1u32, 0u32]).u32();
    if slot_one != 0 {
        e.call(0x0044_59e0, &args![slot_one, 1u32]);
        hide_head = true;
    }
    if part_nodes[1] != 0 {
        let slot_ten = e.call(0x004c_8c10, &args![changes, 10u32, 0u32]).u32();
        if slot_ten != 0 {
            e.call(0x0044_59e0, &args![slot_ten, 1u32]);
            hide_second = false;
            hide_head = true;
        }
    }
    if head_node != 0 {
        e.call(0x0045_0f90, &args![head_node, hide_head]);
    }
    for (index, part) in part_nodes.iter().enumerate() {
        if *part != 0 {
            if index == 0 {
                e.call(0x0045_0f90, &args![*part, hide_head]);
            } else {
                e.call(0x0045_0f90, &args![*part, hide_second]);
            }
        }
    }
}

// Translated from 00606540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the model of an actor (`actor`) with its biped (`biped`). Unless
/// `force` is set, it stops when any of the 20 entries of the biped's part table
/// (`fn_00606800`, entries `0x10` bytes apart) is in use. Then, in this order: the
/// race and sex go to the biped (`004ab250`); the actor's process gets virtual
/// `0x468(1)` unless the extra data `0042e8c0` has an entry (`00441420`) with the
/// first byte 0 and the second not; when the actor has no 3D yet or `force` is
/// set, the default worn items are equipped (`fn_006047c0`) if `005f1590` or
/// `force` allow it and the player's base form being this NPC has nothing worn;
/// `fn_00606820` loads the head unless `004abfa0` says not to; `tesnpc_init_worn`
/// equips the inventory; and `fn_006062e0` runs unless the biped is the player's
/// first-person one (`00950b00(player, 1)`).
///
pub fn fn_00606540(e: &mut Engine, this: Ptr<TESNPC>, actor: Ptr, biped: Ptr, force: u8) {
    if !biped.is_null() {
        if force == 0 {
            for index in 0..0x14u32 {
                let table = fn_00606800(e, biped);
                if e.mem.u32(table + index * 0x10) != 0 {
                    return;
                }
            }
        }
        let female = e.get(this, TESNPC::iActorBaseFlags) & 1 != 0;
        let race = e
            .call(GET_WORD_AT_4, &args![this.addr() + COMPONENT_RACE])
            .u32();
        e.call(0x004a_b250, &args![biped, race, female]);
    }
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![actor, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
        )
        .u32();
    if cast != 0 && process_of(e, cast) != 0 {
        let mut notify = true;
        let owner = e.call(0x005d_43c0, &args![cast]).u32();
        let extra = e.call(0x0042_e8c0, &args![owner]).u32();
        if extra != 0
            && e.call(0x0044_ddc0, &args![extra + 0x20]).u32() != 0
            && e.call(0x0044_1420, &args![extra, 0u32]).u32() != 0
        {
            let mut index = 0u32;
            while index < e.call(0x0044_ddc0, &args![extra + 0x20]).u32() {
                let flags = e.call(0x0044_1420, &args![extra, index]).u32();
                if e.mem.u8(flags) == 0 && e.mem.u8(flags + 1) != 0 {
                    notify = false;
                }
                index += 1;
            }
        }
        if notify {
            let process = process_of(e, cast);
            e.vcall(process, 0x468, &args![1u32]);
        }
    }
    if e.call(0x0043_fcd0, &args![actor]).u32() == 0 || force != 0 {
        let mut equip = e.call(0x005f_1590, &args![this, actor]).u8() | force;
        let player = e.global::<u32>(PLAYER_SINGLETON);
        let player_base = e.call(GET_WORD_AT_20, &args![player]).u32();
        if this.addr() == player_base {
            let changes = e.call(0x004b_f220, &args![actor]).u32();
            if changes != 0 {
                for slot in 0..0x14i32 {
                    let worn = e.call(0x004c_8c10, &args![changes, slot, 0u32]).u32();
                    if worn != 0 {
                        equip = 0;
                        if worn != 0 {
                            e.call(0x0044_59e0, &args![worn, 1u32]);
                        }
                        break;
                    }
                }
            }
        }
        if equip != 0 {
            let mut weapon_out = true;
            let package = e.call(0x0093_44a0, &args![cast]).u32();
            if package != 0 && e.call(0x0044_1b00, &args![package]).bool() {
                weapon_out = false;
            }
            fn_006047c0(e, this, actor, 1, weapon_out, 0, false);
        }
    }
    if e.call(0x004a_bfa0, &args![]).u8() == 0 {
        tesnpc_linear_face_gen_head_load(e, this, actor, biped);
    }
    tesnpc_init_worn(e, this, actor, biped);
    let mut recolour = true;
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if biped.addr() == e.call(0x0095_0b00, &args![player, 1u32]).u32() {
        recolour = false;
    }
    if recolour {
        fn_006062e0(e, this, actor, Ptr::NULL);
    }
}

// Translated from 00606800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the biped part table inside a `BipedAnim` (`this + 0x16c`).
pub fn fn_00606800(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x16c)
}

// Translated from 00606820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::LinearFaceGenHeadLoad` (Xbox PDB): builds the head of an actor
/// (`actor`) from this NPC's `spHeadBiped` and `spHeadSkinned` nodes, or asks the
/// race to make them (`00613c50`) when the NPC has none, and attaches them. Each
/// node is used as it is when the skinned head has at most one reference
/// (`00726070`) and cloned otherwise (`NiCloningProcess` `004ad050`, `Clone`
/// `00a5d2c0`); the children's data are copied (`005495f0`, `00a5d510`) and the
/// skin remapped (`fn_00607310`, `fn_006072c0`, `004adda0`, `004addc0`). The
/// nodes get their flags (virtual `0x114`, `0x11c`), the actor's palette, a
/// parent (the biped's head attach node `004ab230(biped, 0)` for the first, the
/// 3D root for the second) and the actor in `+0xe8`; the first available property
/// is sent virtual `0xb4`. `biped` is the object `004ab230` and `00559450` read.
/// Nothing happens when `00651b30` or `0043faf0` answer 0, or for the player when
/// `00950b30(biped)` is true. A missing attach node or root is logged; an actor
/// that already has the heads (virtual `0x1ac`/`0x1b0`) only has its root updated.
///
/// Not translated: the compiler's exception-unwinding frame.
///
pub fn tesnpc_linear_face_gen_head_load(e: &mut Engine, this: Ptr<TESNPC>, actor: Ptr, biped: Ptr) {
    // Three `NiPointer`s on the stack: the copy target and the two clones.
    e.with_stack(0x14, |e, pointers| {
        let copy = pointers.addr();
        e.call(NI_POINTER_CONSTRUCT, &args![copy, 0u32]);
        head_load_body(e, this, actor, biped, copy);
        e.call(NI_POINTER_DESTROY, &args![copy]);
    });
}

/// The clone of the node `slot` (an `NiPointer` field of the NPC) points at:
/// `NiCloningProcess` (`004ad050(1.0)`), `NiObject::Clone` (`00a5d2c0`), and
/// the process' destructor (`004ad270`).
fn clone_head_node(e: &mut Engine, slot: u32) -> u32 {
    e.with_stack(0x1c, |e, process| {
        e.call(0x004a_d050, &args![process, 1.0f32]);
        let node = e.call(GET_FIRST_WORD, &args![slot]).u32();
        let clone = e.call(0x00a5_d2c0, &args![node, process]).u32();
        e.call(0x004a_d270, &args![process]);
        clone
    })
}

/// Copies a child node's data into `pointer` (`005495f0`, then
/// `NiObject::CreateDeepCopy` `00a5d510` into `copy`), and, when that gives a
/// node, passes it to the child's virtual `0xe4`.
fn copy_child_data(e: &mut Engine, child: u32, copy: u32, pointer: u32) {
    let source = e.call(0x0054_95f0, &args![child]).u32();
    e.call(0x00a5_d510, &args![source, copy]);
    let value = e.call(GET_FIRST_WORD, &args![copy]).u32();
    e.call(NI_POINTER_ASSIGN, &args![pointer, value]);
    if e.call(GET_FIRST_WORD, &args![pointer]).u32() != 0 {
        let held = e.call(GET_FIRST_WORD, &args![pointer]).u32();
        e.vcall(child, 0xe4, &args![held]);
    }
}

/// The part of `LinearFaceGenHeadLoad` after the `NiPointer` for the copy
/// target (`copy`) is built; `copy + 4` and `copy + 8` are the other two.
fn head_load_body(e: &mut Engine, this: Ptr<TESNPC>, actor: Ptr, biped: Ptr, copy: u32) {
    let base = this.addr();
    let pointer_a = copy + 4;
    let pointer_b = copy + 8;
    if e.call(0x0065_1b30, &args![]).u32() == 0 {
        return;
    }
    if e.call(GET_FIRST_WORD, &args![biped]).u32() == 0 {
        return;
    }
    if e.call(0x0043_faf0, &args![]).u8() == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if actor.addr() == player && e.call(0x0095_0b30, &args![player, biped]).bool() {
        return;
    }
    let node = e.call(0x0043_fcd0, &args![actor]).u32();
    let mut root = 0;
    if node != 0 {
        root = e.vcall(node, 0xc, &args![]).u32();
    }
    let attach = e.call(0x004a_b230, &args![biped, 0u32]).u32();
    e.with_stack(0x24, |e, scratch| {
        e.call(LIST_NODE_SELF, &args![scratch]);
    });
    let mut palette = 0;
    if e.vcall(actor.addr(), 0x1e4, &args![]).u32() != 0 {
        let a = e.vcall(actor.addr(), 0x1e4, &args![]).u32();
        if e.call(0x0049_6940, &args![a]).u32() != 0 {
            let a = e.vcall(actor.addr(), 0x1e4, &args![]).u32();
            let b = e.call(0x0049_6940, &args![a]).u32();
            palette = e.call(0x0053_7bd0, &args![b]).u32();
        }
    }
    if attach == 0 || root == 0 {
        let id = e.call(GET_FORM_ID, &args![this]).u32();
        e.call(LOG_MESSAGE, &args![0x0104_a860u32, id]);
        return;
    }
    let attached = e.vcall(actor.addr(), 0x1ac, &args![attach]).u32() != 0
        || e.vcall(actor.addr(), 0x1b0, &args![attach]).u32() != 0;
    if !attached {
        let mut head_a = 0u32;
        let mut head_b = 0u32;
        let mut selected = 0u32;
        e.call(NI_POINTER_CONSTRUCT, &args![pointer_a, 0u32]);
        e.call(NI_POINTER_CONSTRUCT, &args![pointer_b, 0u32]);
        if e.call(GET_FIRST_WORD, &args![base + 0x1c4]).u32() != 0 {
            let skinned = e.call(GET_FIRST_WORD, &args![base + 0x1c8]).u32();
            let mut reuse = false;
            if skinned != 0 {
                let skinned = e.call(GET_FIRST_WORD, &args![base + 0x1c8]).u32();
                reuse = e.call(GET_WORD_AT_4, &args![skinned]).u32() <= 1;
            }
            if reuse {
                head_a = e.call(GET_FIRST_WORD, &args![base + 0x1c4]).u32();
            } else {
                head_a = clone_head_node(e, base + 0x1c4);
            }
            let count = e.call(0x0043_b480, &args![head_a]).u32();
            for index in 0..count {
                let child = e.call(0x0043_b4a0, &args![head_a, index]).u32();
                let cast = if child != 0 {
                    e.vcall(child, 0x1c, &args![]).u32()
                } else {
                    0
                };
                if cast != 0 && e.call(0x0052_aa80, &args![base + 0x1c4, head_a]).bool() {
                    copy_child_data(e, cast, copy, pointer_a);
                }
            }
            if e.call(0x0052_aa80, &args![base + 0x1c4, head_a]).bool() {
                let property = e.vcall(head_a, 0x100, &args![]).u32();
                selected = e.call(0x0064_c5a0, &args![property]).u32();
                if selected != 0 {
                    e.vcall(head_a, 0x104, &args![selected]);
                }
            }
        }
        if e.call(GET_FIRST_WORD, &args![base + 0x1c8]).u32() != 0 {
            let skinned = e.call(GET_FIRST_WORD, &args![base + 0x1c8]).u32();
            if e.call(GET_WORD_AT_4, &args![skinned]).u32() <= 1 {
                head_b = e.call(GET_FIRST_WORD, &args![base + 0x1c8]).u32();
            } else {
                head_b = clone_head_node(e, base + 0x1c8);
            }
            let count = e.call(0x0043_b480, &args![head_b]).u32();
            for index in 0..count {
                let child = e.call(0x0043_b4a0, &args![head_b, index]).u32();
                let cast = if child != 0 {
                    e.vcall(child, 0x1c, &args![]).u32()
                } else {
                    0
                };
                if cast == 0 {
                    continue;
                }
                if e.call(0x0052_aa80, &args![base + 0x1c8, head_b]).bool() {
                    copy_child_data(e, cast, copy, pointer_a);
                }
                if e.call(0x0043_fad0, &args![cast]).u32() != 0 {
                    let skin = e.call(0x0043_fad0, &args![cast]).u32();
                    if e.call(0x0043_b230, &args![skin]).u32() != 0 {
                        let remapper = fn_00607310(e, cast);
                        let mut remappable = false;
                        if remapper != 0 && e.vcall(remapper, 0x94, &args![]).u32() != 0 {
                            let table = e.vcall(remapper, 0x94, &args![]).u32();
                            remappable = fn_006072c0(e, Ptr::new(table)) != 0;
                        }
                        if remappable {
                            let table = e.vcall(remapper, 0x94, &args![]).u32();
                            let entries = fn_006072c0(e, Ptr::new(table));
                            let skin = e.call(0x0043_fad0, &args![cast]).u32();
                            let values = e.call(0x0082_5c00, &args![entries]).u32();
                            let list = e.call(0x0055_85e0, &args![entries]).u32();
                            let total = e.call(0x0080_41a0, &args![list]).u32();
                            for position in 0..total {
                                let value = e.mem.u32(values + position * 4);
                                e.call(0x004a_dda0, &args![skin, position, value]);
                            }
                        } else {
                            let id = e.call(GET_FORM_ID, &args![this]).u32();
                            let name = e.vcall(this.addr(), VSLOT_GET_FORM_NAME, &args![]).u32();
                            e.call(LOG_MESSAGE, &args![0x0104_a908u32, name, id]);
                        }
                        let skin = e.call(0x0043_fad0, &args![cast]).u32();
                        let shape = e.call(0x0043_b230, &args![skin]).u32();
                        e.call(0x00a5_d510, &args![shape, copy]);
                        let value = e.call(GET_FIRST_WORD, &args![copy]).u32();
                        e.call(NI_POINTER_ASSIGN, &args![pointer_b, value]);
                        if e.call(GET_FIRST_WORD, &args![pointer_b]).u32() != 0 {
                            let held = e.call(GET_FIRST_WORD, &args![pointer_b]).u32();
                            let skin = e.call(0x0043_fad0, &args![cast]).u32();
                            e.call(0x004a_ddc0, &args![skin, held]);
                        }
                    }
                }
            }
            if e.call(0x0052_aa80, &args![base + 0x1c8, head_b]).bool() {
                if selected == 0 {
                    let property = e.vcall(head_b, 0x100, &args![]).u32();
                    selected = e.call(0x0064_c5a0, &args![property]).u32();
                    if selected != 0 {
                        e.vcall(head_b, 0x104, &args![selected]);
                    }
                } else {
                    e.vcall(head_b, 0x104, &args![selected]);
                }
            }
        }
        if e.call(GET_FIRST_WORD, &args![base + 0x1c4]).u32() == 0
            && e.call(GET_FIRST_WORD, &args![base + 0x1c8]).u32() == 0
            && e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32() != 0
        {
            let cell_a = copy + 0xc;
            let cell_b = copy + 0x10;
            e.mem.set_u32(cell_a, head_a);
            e.mem.set_u32(cell_b, head_b);
            let race = e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32();
            e.call(
                0x0061_3c50,
                &args![race, cell_a, cell_b, this, 0u32, 0u32, 0u32],
            );
            head_a = e.mem.u32(cell_a);
            head_b = e.mem.u32(cell_b);
            let race = e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32();
            let face_number = fn_00607350(e, Ptr::new(race));
            e.set(this, TESNPC::sLastRaceFaceNum, face_number);
            e.call(NI_POINTER_ASSIGN, &args![base + 0x1c4, head_a]);
            e.call(NI_POINTER_ASSIGN, &args![base + 0x1c8, head_b]);
        }
        if head_a == 0 && head_b == 0 {
            let id = e.call(GET_FORM_ID, &args![this]).u32();
            e.call(LOG_MESSAGE, &args![0x0104_a8b8u32, id]);
        }
        if head_a != 0 {
            if e.vcall(head_a, 0x100, &args![]).u32() != 0 {
                let scale = e.call(0x0056_8ad0, &args![actor]).f32();
                if f64::from(scale) <= e.global::<f64>(ZERO_DOUBLE) {
                    let property = e.vcall(head_a, 0x100, &args![]).u32();
                    e.vcall(property, 0xd8, &args![1u32, 1u32]);
                    let property = e.vcall(head_a, 0x100, &args![]).u32();
                    e.vcall(property, 0xd0, &args![1u32]);
                }
            }
            e.vcall(head_a, 0x114, &args![1u32]);
            e.vcall(head_a, 0x11c, &args![1u32]);
            e.call(0x0044_0460, &args![head_a, 0x011f_426cu32]);
            e.call(0x0043_fa80, &args![head_a, 0x011a_9448u32]);
            e.vcall(attach, 0xdc, &args![head_a, 1u32]);
            e.mem.set_u32(head_a + 0xe8, actor.addr());
            e.call(0x00a6_e870, &args![head_a, palette]);
        }
        if head_b != 0 {
            let no_a = head_a == 0;
            e.vcall(head_b, 0x114, &args![no_a]);
            e.vcall(head_b, 0x11c, &args![no_a]);
            e.call(0x0044_0460, &args![head_b, 0x011f_426cu32]);
            e.vcall(root, 0xdc, &args![head_b, 1u32]);
            e.mem.set_u32(head_b + 0xe8, actor.addr());
            e.call(0x00a6_e870, &args![head_b, palette]);
            e.vcall(head_b, 0x128, &args![root, 1u32]);
        }
        let mut target = if head_a != 0 {
            e.vcall(head_a, 0x100, &args![]).u32()
        } else {
            0
        };
        if target == 0 {
            target = if head_b != 0 {
                e.vcall(head_b, 0x100, &args![]).u32()
            } else {
                0
            };
        }
        if target != 0 {
            e.vcall(target, 0xb4, &args![0.0f32, 1u32, 1u32, 1u32, 1u32, 0u32]);
        }
        e.call(NI_POINTER_DESTROY, &args![pointer_b]);
        e.call(NI_POINTER_DESTROY, &args![pointer_a]);
    }
    update_node(e, root);
}

// Translated from 006072c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The skin data node of the object behind the `NiPointer` at `this + 0x10`
/// of the word at `this + 8`: `0043fad0` of the pointer's target, or 0.
pub fn fn_006072c0(e: &mut Engine, this: Ptr) -> u32 {
    let inner = e.mem.u32(this.addr() + 8);
    if inner != 0 && e.call(GET_FIRST_WORD, &args![inner + 0x10]).u32() != 0 {
        let target = e.call(GET_FIRST_WORD, &args![inner + 0x10]).u32();
        e.call(0x0043_fad0, &args![target]).u32()
    } else {
        0
    }
}

// Translated from 00607310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00a5bdd0(node, fn_00607340())` for a non-null `node` (an
/// `NiObject::GetExtraData`-style lookup by the key of `fn_00607340`), 0 for
/// null.
pub fn fn_00607310(e: &mut Engine, node: u32) -> u32 {
    if node != 0 {
        let key = fn_00607340(e);
        e.call(0x00a5_bdd0, &args![node, key]).u32()
    } else {
        0
    }
}

// Translated from 00607340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at the exe global `011d5b50`.
pub fn fn_00607340(e: &mut Engine) -> u32 {
    e.global(0x011d_5b50)
}

// Translated from 00607350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `u16` at `+0x4f8` of a race.
pub fn fn_00607350(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x4f8)
}

// Translated from 00607370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::InitHead` (Xbox PDB): clears both head `NiPointer`s and, when
/// they stay empty and the NPC has a race, asks the race to make the head
/// nodes (`00613c50(race, first, second, this, 1, 0, 0)`) and stores the
/// race's `+0x4f8` number in `sLastRaceFaceNum`.
pub fn tesnpc_init_head(e: &mut Engine, this: Ptr<TESNPC>, first: Ptr, second: Ptr) {
    let base = this.addr();
    e.call(NI_POINTER_ASSIGN, &args![base + 0x1c4, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![base + 0x1c8, 0u32]);
    if e.call(GET_FIRST_WORD, &args![base + 0x1c4]).u32() == 0
        && e.call(GET_FIRST_WORD, &args![base + 0x1c8]).u32() == 0
        && e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32() != 0
    {
        let race = e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32();
        e.call(
            0x0061_3c50,
            &args![race, first, second, this, 1u32, 0u32, 0u32],
        );
        let race = e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32();
        let face_number = fn_00607350(e, Ptr::new(race));
        e.set(this, TESNPC::sLastRaceFaceNum, face_number);
    }
}

// Translated from 00607420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attaches two head nodes (`head_a`, `head_b`; either may be null) to an
/// actor's model, as `LinearFaceGenHeadLoad` does for the nodes it builds:
/// `head_a` hangs off the node `004ab230(biped, 0)` gives, `head_b` off the 3D
/// root's virtual `0xc` node. Both get their flags (virtual `0x114`, `0x11c`), the
/// palette of the actor (virtual `0x1e4`, `00a6e870`), and `head_a` also the
/// shared transform `0043fa80(0x011a9448)` and, when its virtual `0x100` property
/// exists and `00568ad0(actor)` is not above 0.0, the property flags (virtual
/// `0xd8`, `0xd0`). The first available property is then sent virtual `0xb4`;
/// when the actor's virtual `0x100` is true and `+0xac` is set, four values from
/// `00649f00`, `00649f70`, `00649fe0` and `0064a070` are stored in that object
/// (`fn_00607830`, `fn_00607810`, `00c748d0`). `spHeadBiped` and `spHeadSkinned`
/// are set to the two nodes and the root is updated. Without the biped's attach
/// node or the root the form id is logged. Ends with `fn_006062e0(this, actor, 0)`.
///
pub fn fn_00607420(
    e: &mut Engine,
    this: Ptr<TESNPC>,
    actor: Ptr,
    biped: Ptr,
    head_a: Ptr,
    head_b: Ptr,
) {
    let base = this.addr();
    let (head_a, head_b) = (head_a.addr(), head_b.addr());
    let node = e.call(0x0043_fcd0, &args![actor]).u32();
    let mut root = 0;
    if node != 0 {
        root = e.vcall(node, 0xc, &args![]).u32();
    }
    let attach = e.call(0x004a_b230, &args![biped, 0u32]).u32();
    e.with_stack(0x24, |e, scratch| {
        e.call(LIST_NODE_SELF, &args![scratch]);
    });
    let mut palette = 0;
    if e.vcall(actor.addr(), 0x1e4, &args![]).u32() != 0 {
        let a = e.vcall(actor.addr(), 0x1e4, &args![]).u32();
        if e.call(0x0049_6940, &args![a]).u32() != 0 {
            let a = e.vcall(actor.addr(), 0x1e4, &args![]).u32();
            let b = e.call(0x0049_6940, &args![a]).u32();
            palette = e.call(0x0053_7bd0, &args![b]).u32();
        }
    }
    if attach == 0 || root == 0 {
        let id = e.call(GET_FORM_ID, &args![this]).u32();
        e.call(LOG_MESSAGE, &args![0x0104_a860u32, id]);
    } else {
        if head_a != 0 {
            if e.vcall(head_a, 0x100, &args![]).u32() != 0 {
                let scale = e.call(0x0056_8ad0, &args![actor]).f32();
                if f64::from(scale) <= e.global::<f64>(ZERO_DOUBLE) {
                    let property = e.vcall(head_a, 0x100, &args![]).u32();
                    e.vcall(property, 0xd8, &args![1u32, 1u32]);
                    let property = e.vcall(head_a, 0x100, &args![]).u32();
                    e.vcall(property, 0xd0, &args![1u32]);
                }
            }
            e.vcall(head_a, 0x114, &args![1u32]);
            e.vcall(head_a, 0x11c, &args![1u32]);
            e.call(0x0044_0460, &args![head_a, 0x011f_426cu32]);
            e.call(0x0043_fa80, &args![head_a, 0x011a_9448u32]);
            e.vcall(attach, 0xdc, &args![head_a, 1u32]);
            e.call(0x00a6_e870, &args![head_a, palette]);
        }
        if head_b != 0 {
            let no_a = head_a == 0;
            e.vcall(head_b, 0x114, &args![no_a]);
            e.vcall(head_b, 0x11c, &args![no_a]);
            e.call(0x0044_0460, &args![head_b, 0x011f_426cu32]);
            e.vcall(root, 0xdc, &args![head_b, 1u32]);
            e.call(0x00a6_e870, &args![head_b, palette]);
            e.vcall(head_b, 0x128, &args![root, 1u32]);
        }
        let mut target = if head_a != 0 {
            e.vcall(head_a, 0x100, &args![]).u32()
        } else {
            0
        };
        if target == 0 {
            target = if head_b != 0 {
                e.vcall(head_b, 0x100, &args![]).u32()
            } else {
                0
            };
        }
        if target != 0 {
            e.vcall(target, 0xb4, &args![0.0f32, 1u32, 1u32, 1u32, 1u32, 0u32]);
            if e.vcall(actor.addr(), 0x100, &args![]).bool() {
                let table = e.mem.u32(actor.addr() + 0xac);
                if table != 0 {
                    e.with_stack(0xc, |e, values| {
                        let (first, second, third) =
                            (values.addr(), values.addr() + 4, values.addr() + 8);
                        e.call(0x0064_9f00, &args![first, second]);
                        e.call(0x0064_9f70, &args![first, third]);
                        let value = e.mem.f32(third);
                        fn_00607830(e, Ptr::new(table), value);
                        let value = e.mem.f32(second);
                        fn_00607810(e, Ptr::new(table), 0, value);
                        e.call(0x0064_9fe0, &args![first, second]);
                        e.call(0x0064_a070, &args![first, third]);
                        let (low, high) = (e.mem.f32(second), e.mem.f32(third));
                        e.call(0x00c7_48d0, &args![table, 0u32, low, high]);
                    });
                }
            }
        }
        e.call(NI_POINTER_ASSIGN, &args![base + 0x1c4, head_a]);
        e.call(NI_POINTER_ASSIGN, &args![base + 0x1c8, head_b]);
        refresh_properties_and_update(e, root);
    }
    fn_006062e0(e, this, actor, Ptr::NULL);
}

/// `UpdateProperties` (`00a5a040`) then [`update_node`].
fn refresh_properties_and_update(e: &mut Engine, node: u32) {
    e.call(0x00a5_a040, &args![node]);
    update_node(e, node);
}

// Translated from 00607810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `float` in the 80-byte record `index` of the array at `+0xf8`.
pub fn fn_00607810(e: &mut Engine, this: Ptr, index: u32, value: f32) {
    e.mem
        .set_f32(this.addr() + index.wrapping_mul(0x50) + 0xf8, value);
}

// Translated from 00607830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `float` at `+0x1a8`.
pub fn fn_00607830(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x1a8, x87_quiet(value));
}

// Translated from 00607850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of one actor value for the NPC (`index`): for an index
/// `0047f060` accepts, `cSkill[i] + cOffset[i]` where `i` is
/// `0066ec80(2, index)` (the offset only when the NPC is not auto-calculated,
/// virtual `0x144` of the NPC); for the others `005f0fb0(this, index)`.
///
/// `this` is the NPC's `+0x100` address (the exe reads `data` at `this + 0x14`
/// and `this - 0x100` is the NPC itself).
pub fn fn_00607850(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if e.call(0x0047_f060, &args![index]).bool() {
        let skill = e.call(0x0066_ec80, &args![2u32, index]).u8() as i8 as i32 as u32;
        let mut value = e.mem.u8(this.addr().wrapping_add(skill).wrapping_add(0x14)) as u32;
        let npc = this.addr().wrapping_sub(0x100);
        if !e.vcall(npc, 0x144, &args![]).bool() {
            value += e.mem.u8(this.addr().wrapping_add(skill).wrapping_add(0x22)) as u32;
        }
        value
    } else {
        e.call(0x005f_0fb0, &args![this, index]).u32()
    }
}

// Translated from 006078e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets one actor value of the NPC: for an `index` that `0047f060` accepts,
/// the skill `0066ec80(2, index)` is set to `value`'s low byte and the NPC is
/// told to update (virtual `0x48(0x200)`); otherwise `005f12d0(index,
/// value)` does it.
pub fn fn_006078e0(e: &mut Engine, this: Ptr<TESNPC>, index: u32, value: u32) {
    if e.call(0x0047_f060, &args![index]).bool() {
        let skill = e.call(0x0066_ec80, &args![2u32, index]).u8() as i8 as i32 as u32;
        e.mem.set_u8(
            this.addr().wrapping_add(skill).wrapping_add(0x114),
            value as u8,
        );
        e.vcall(this.addr(), 0x48, &args![0x200u32]);
    } else {
        e.call(0x005f_12d0, &args![this, index, value]);
    }
}

// Translated from 00607950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pCombatStyle` (the word at `+0x1d4`).
pub fn fn_00607950(e: &mut Engine, this: Ptr<TESNPC>) -> Ptr {
    e.get(this, TESNPC::pCombatStyle)
}

// Translated from 00607970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `pCombatStyle`.
pub fn fn_00607970(e: &mut Engine, this: Ptr<TESNPC>, combat_style: Ptr) {
    e.set(this, TESNPC::pCombatStyle, combat_style);
}

// Translated from 00608d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `004f8960(this)` is 4.
pub fn fn_00608d80(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x004f_8960, &args![this]).u32() == 4
}

// Translated from 00608da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size of the NPC's save data for the change flags `changes`: the base
/// size (`005f16f0`) plus `0xe` when bit `0x200` (the `NPC_DATA` block) is
/// set. (The exe also tests `changes & 0` for a further 4 bytes; that test
/// can never pass.)
pub fn fn_00608da0(e: &mut Engine, this: Ptr<TESNPC>, changes: u32) -> u16 {
    let mut size = e.call(0x005f_16f0, &args![this, changes]).u16();
    if changes & 0x200 != 0 {
        size = size.wrapping_add(0xe);
    }
    size
}

// Translated from 00608e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the NPC's save data for the change flags `changes`: the base data
/// (`005f18c0`), then the first `0xe` bytes of `NPC_DATA` when bit `0x200` is
/// set (`00484ce0`). (A further block guarded by `changes & 0` can never be
/// written and is not translated.)
pub fn fn_00608e00(e: &mut Engine, this: Ptr<TESNPC>, changes: u32) {
    e.call(0x005f_18c0, &args![this, changes]);
    if changes & 0x200 != 0 {
        e.call(0x0048_4ce0, &args![this, this.addr() + 0x114, 0xeu32]);
    }
}

// Translated from 00608e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the NPC's save data for the change flags `changes` (`005f1b30` with
/// `extra`), then the first `0xe` bytes of `NPC_DATA` when bit `0x200` is set
/// (`00484d00`). (A block guarded by `changes & 0` can never run and is not
/// translated.)
pub fn fn_00608e80(e: &mut Engine, this: Ptr<TESNPC>, changes: u32, extra: u32) {
    e.call(0x005f_1b30, &args![this, changes, extra]);
    if changes & 0x200 != 0 {
        e.call(0x0048_4d00, &args![this, this.addr() + 0x114, 0xeu32]);
    }
}

// Translated from 00608f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes this NPC's changes to a save buffer (`buffer`; `BGSSaveFormBuffer`):
/// the base form (`005f1f30`), then for each change flag set in the buffer
/// (tested with `change_flag_set(00428110, ..)`): `0x200` the `NPC_DATA`
/// (`0x1c` bytes); `0x400` the class; `0x2000000` the race and the original
/// race; `0x800` the face: a flag byte (1 when the alternate coordinate is
/// used), the 2 x 2 matrices of the coordinate in use, hair, eyes, hair
/// length, hair colour and the head part list as a counted run of form ids;
/// `0x1000000` the sex bit of the actor flags.
pub fn fn_00608f00(e: &mut Engine, this: Ptr<TESNPC>, buffer: Ptr) {
    const SAVE_BYTES: u32 = 0x0086_5e50;
    const SAVE_FORM_ID: u32 = 0x0086_5df0;
    let base = this.addr();
    e.call(0x005f_1f30, &args![this, buffer]);
    if change_flag_set(e, 0x0042_8110, buffer, 0x200) {
        e.call(SAVE_BYTES, &args![buffer, base + 0x114, 0x1cu32, 0u32]);
    }
    if change_flag_set(e, 0x0042_8110, buffer, 0x400) {
        let class = e.get(this, TESNPC::pCl);
        e.call(SAVE_FORM_ID, &args![buffer, class, 0u32]);
    }
    if change_flag_set(e, 0x0042_8110, buffer, 0x200_0000) {
        let race = e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32();
        e.call(SAVE_FORM_ID, &args![buffer, race, 0u32]);
        let original = e.get(this, TESNPC::pOriginalRace);
        e.call(SAVE_FORM_ID, &args![buffer, original, 0u32]);
    }
    if change_flag_set(e, 0x0042_8110, buffer, 0x800) {
        e.with_stack(0x14, |e, locals| {
            let flag = locals.addr();
            let iterator = locals.addr() + 4;
            let value = locals.addr() + 0xc;
            let mut coordinate = base + 0x134;
            e.mem.set_u8(flag, 0);
            let alternate = e.get(this, TESNPC::pAlternateFaceOffsetCoord);
            if !alternate.is_null() {
                coordinate = alternate.addr();
                e.mem.set_u8(flag, 1);
            }
            e.call(SAVE_BYTES, &args![buffer, flag, 1u32, 0u32]);
            for row in 0..2u32 {
                for column in 0..2u32 {
                    let matrix = coordinate + row * 0x40 + column * 0x20;
                    let width = e.call(0x0096_11e0, &args![matrix]).u32();
                    let height = e.call(0x0044_1110, &args![matrix]).u32();
                    for x in 0..width {
                        for y in 0..height {
                            let at = e
                                .call(MATRIX_ITERATOR_AT, &args![matrix, iterator, x])
                                .u32();
                            let element = e.call(ITERATOR_ELEMENT, &args![at, y]).u32();
                            let word = e.mem.u32(element);
                            e.mem.set_u32(value, word);
                            e.call(SAVE_BYTES, &args![buffer, value, 4u32, 0u32]);
                        }
                    }
                }
            }
        });
        let hair = e.get(this, TESNPC::pHair);
        e.call(SAVE_FORM_ID, &args![buffer, hair, 0u32]);
        let eyes = e.get(this, TESNPC::pEyeColor);
        e.call(SAVE_FORM_ID, &args![buffer, eyes, 0u32]);
        e.call(SAVE_BYTES, &args![buffer, base + 0x1bc, 4u32, 0u32]);
        e.call(SAVE_BYTES, &args![buffer, base + 0x1d8, 4u32, 0u32]);
        let mut count = 0u32;
        let token = e.call(0x0086_5f20, &args![buffer]).u32();
        let mut node = base + 0x1dc;
        while node != 0 {
            let cell = e.call(LIST_NODE_SELF, &args![node]).u32();
            let part = e.mem.u32(cell);
            if part != 0 {
                e.call(SAVE_FORM_ID, &args![buffer, part, 0u32]);
                count += 1;
            }
            node = e.call(GET_WORD_AT_4, &args![node]).u32();
        }
        e.call(0x0086_5ff0, &args![buffer, count, token]);
    }
    if change_flag_set(e, 0x0042_8110, buffer, 0x100_0000) {
        e.with_stack(4, |e, flag| {
            let bit = e.call(
                TEST_ACTOR_FLAGS,
                &args![base + COMPONENT_ACTOR_BASE_DATA, 1u32],
            );
            e.mem.set_u8(flag.addr(), bit.u8());
            e.call(SAVE_BYTES, &args![buffer, flag, 1u32, 0u32]);
        });
    }
}

// Translated from 00609220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads this NPC's changes from a load buffer (`buffer`) and applies what
/// differs, collecting a mask of what changed (`0x1b` for the race and the
/// sex bit, `8` for the face) that is handed to `005f20a0`; clears the head
/// (`005dd560`) when the face changed. The flags tested are those of
/// `fn_00608f00`: `0x200` `NPC_DATA`, `0x400` class, `0x2000000` race (a
/// changed race goes through `006ecd40`) and original race, `0x800` the face
/// (a coordinate that is read for the alternate set is created and
/// initialised on demand; the head part list is rebuilt, and compared for
/// the `8` bit when the buffer version (virtual `0`) is at least `0xe`),
/// `0x1000000` the sex bit (`0047dd50`). A height equal to the race height
/// follows the race.
pub fn fn_00609220(e: &mut Engine, this: Ptr<TESNPC>, buffer: Ptr) {
    const LOAD_BYTES: u32 = 0x0086_4980;
    let base = this.addr();
    let mut changed: u8 = 0;
    e.call(0x005f_1fd0, &args![this, buffer]);
    if change_flag_set(e, 0x0042_8110, buffer, 0x200) {
        e.call(LOAD_BYTES, &args![buffer, base + 0x114, 0x1cu32]);
    }
    if change_flag_set(e, 0x0042_8110, buffer, 0x400) {
        let class = load_form_cast(e, buffer, TYPE_TES_CLASS);
        e.set(this, TESNPC::pCl, Ptr::new(class));
    }
    if e.call(0x0042_ce90, &args![buffer]).bool() {
        changed |= 0x1b;
    }
    let mut follows_race = false;
    let race_height = tesnpc_get_race_height(e, this);
    if e.get(this, TESNPC::fHeight) == race_height {
        follows_race = true;
    }
    if change_flag_set(e, 0x0042_8110, buffer, 0x200_0000) {
        let race = load_form_cast(e, buffer, TYPE_TES_RACE);
        if race != e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32() {
            e.call(SET_WORD_AT_4, &args![base + COMPONENT_RACE, race]);
            changed |= 0x1b;
        }
        let original = load_form_cast(e, buffer, TYPE_TES_RACE);
        e.set(this, TESNPC::pOriginalRace, Ptr::new(original));
    }
    if follows_race {
        let height = tesnpc_get_race_height(e, this);
        e.set(this, TESNPC::fHeight, height);
    }
    if change_flag_set(e, 0x0042_8110, buffer, 0x800) {
        load_face(e, this, buffer, &mut changed);
    }
    if change_flag_set(e, 0x0042_8110, buffer, 0x100_0000) {
        let stored = e.with_stack(4, |e, flag| {
            e.mem.set_u8(flag.addr(), 0);
            e.call(LOAD_BYTES, &args![buffer, flag, 1u32]);
            e.mem.u8(flag.addr())
        });
        let current = e
            .call(
                TEST_ACTOR_FLAGS,
                &args![base + COMPONENT_ACTOR_BASE_DATA, 1u32],
            )
            .u8();
        if current != stored {
            e.call(
                0x0047_dd50,
                &args![base + COMPONENT_ACTOR_BASE_DATA, 1u32, stored as u32, 1u32],
            );
            changed |= 0x1b;
        }
    }
    if changed & 8 != 0 {
        e.call(0x005d_d560, &args![this]);
    }
    if changed != 0 {
        e.call(0x005f_20a0, &args![this, changed as u32]);
    }
}

/// The `0x800` (face) block of `fn_00609220`.
fn load_face(e: &mut Engine, this: Ptr<TESNPC>, buffer: Ptr, changed: &mut u8) {
    const LOAD_BYTES: u32 = 0x0086_4980;
    let base = this.addr();
    let uses_alternate = e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), 0);
        e.call(LOAD_BYTES, &args![buffer, flag, 1u32]);
        e.mem.u8(flag.addr()) != 0
    });
    let mut coordinate = base + 0x134;
    if uses_alternate {
        if e.get(this, TESNPC::pAlternateFaceOffsetCoord).is_null() {
            // `new FaceGenCoord[4]` with its element count in front.
            let memory = e.call(OPERATOR_NEW, &args![0x84u32]).u32();
            let array = if memory != 0 {
                e.mem.set_u32(memory, 4);
                e.call(
                    VECTOR_CONSTRUCTOR_ITERATOR,
                    &args![memory + 4, 0x20u32, 4u32, 0x0044_9610u32, 0x0044_9680u32],
                );
                memory + 4
            } else {
                0
            };
            e.set(this, TESNPC::pAlternateFaceOffsetCoord, Ptr::new(array));
            let alternate = e.get(this, TESNPC::pAlternateFaceOffsetCoord);
            e.call(FACEGEN_INIT_COORD, &args![alternate]);
        }
        coordinate = e.get(this, TESNPC::pAlternateFaceOffsetCoord).addr();
    }
    e.with_stack(0x10, |e, locals| {
        let value = locals.addr();
        let iterator = locals.addr() + 4;
        for row in 0..2u32 {
            for column in 0..2u32 {
                let matrix = coordinate + row * 0x40 + column * 0x20;
                let width = e.call(0x0096_11e0, &args![matrix]).u32();
                let height = e.call(0x0044_1110, &args![matrix]).u32();
                for x in 0..width {
                    for y in 0..height {
                        e.mem.set_f32(value, 0.0);
                        e.call(LOAD_BYTES, &args![buffer, value, 4u32]);
                        let at = e
                            .call(MATRIX_ITERATOR_AT, &args![matrix, iterator, x])
                            .u32();
                        let element = e.call(ITERATOR_ELEMENT, &args![at, y]).u32();
                        let read = e.mem.f32(value);
                        if e.mem.f32(element) != read {
                            let at = e
                                .call(MATRIX_ITERATOR_AT, &args![matrix, iterator, x])
                                .u32();
                            let element = e.call(ITERATOR_ELEMENT, &args![at, y]).u32();
                            e.mem.set_f32(element, read);
                            *changed |= 8;
                        }
                    }
                }
            }
        }
    });
    let old_hair = e.get(this, TESNPC::pHair);
    let old_eyes = e.get(this, TESNPC::pEyeColor);
    let old_length = e.get(this, TESNPC::fHairLength);
    let old_color = e.get(this, TESNPC::iHairColor);
    // The old colour is kept as a `float` (`FILD`, then `FSTP` to a `float`).
    let old_color_float = old_color as f64 as f32;
    let hair = load_form_cast(e, buffer, TYPE_TES_HAIR);
    e.set(this, TESNPC::pHair, Ptr::new(hair));
    let eyes = load_form_cast(e, buffer, TYPE_TES_EYES);
    e.set(this, TESNPC::pEyeColor, Ptr::new(eyes));
    e.call(LOAD_BYTES, &args![buffer, base + 0x1bc, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, base + 0x1d8, 4u32]);
    let mut parts_differ = false;
    let version = e.vcall(buffer.addr(), 0, &args![]).u8();
    if version >= 0xe {
        // The scrap array is 0x14 bytes (its scrap heap at +0x10, set by 0060ba40), so the
        // cell follows it. In the exe they are separate locals: the array at EBP-0x5c, the
        // cells at EBP-0x64 and EBP-0x74.
        e.with_stack(0x18, |e, locals| {
            let array = locals.addr();
            let cell = locals.addr() + 0x14;
            e.call(0x0060_ba40, &args![array]);
            let mut node = base + 0x1dc;
            while node != 0 {
                let item_cell = e.call(LIST_NODE_SELF, &args![node]).u32();
                let part = e.mem.u32(item_cell);
                e.mem.set_u32(cell, part);
                if part != 0 {
                    e.call(0x007c_b2e0, &args![array, cell]);
                }
                node = e.call(GET_WORD_AT_4, &args![node]).u32();
            }
            e.call(0x0047_0470, &args![base + 0x1dc]);
            let count = e.call(0x0086_4a60, &args![buffer]).u32();
            for _ in 0..count {
                let part = load_form_cast(e, buffer, TYPE_BGS_HEAD_PART);
                e.mem.set_u32(cell, part);
                if part != 0 {
                    e.call(0x005a_e3d0, &args![base + 0x1dc, cell]);
                }
            }
            let mut node = base + 0x1dc;
            while node != 0 {
                let item_cell = e.call(LIST_NODE_SELF, &args![node]).u32();
                let part = e.mem.u32(item_cell);
                e.mem.set_u32(cell, part);
                if part != 0 {
                    let index = e
                        .call(0x0071_9b20, &args![array, cell, 0u32, 0x009a_3830u32])
                        .i32();
                    if index == -1 {
                        parts_differ = true;
                        break;
                    }
                    e.call(0x009a_4320, &args![array, index, 1u32]);
                }
                node = e.call(GET_WORD_AT_4, &args![node]).u32();
            }
            if e.call(0x0044_ddc0, &args![array]).u32() != 0 {
                parts_differ = true;
            }
            e.call(0x0060_bae0, &args![array]);
        });
    }
    let new_color = e.get(this, TESNPC::iHairColor);
    if parts_differ
        || old_hair != e.get(this, TESNPC::pHair)
        || old_eyes != e.get(this, TESNPC::pEyeColor)
        || old_length != e.get(this, TESNPC::fHairLength)
        || f64::from(new_color) != f64::from(old_color_float)
    {
        *changed |= 8;
    }
}

// Translated from 006099f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The counterpart of `fn_00609220` for a buffer whose first flag object
/// (`0042ce30`) has a change flag that the second (`00428110`) lacks, after the
/// base form's step (`009dace0`): without `0x2000000` in the second, a set
/// original race goes back to the race component (`006ecd40`) and is cleared;
/// without `0x800`, the alternate coordinate is destroyed
/// (`005d9ff0(coordinate, 3)`) and cleared and the head part list emptied
/// (`00470470`); without `0x1000000`, the sex bit is flipped (`0047dd50`). A height
/// equal to the race height follows the race. The changed mask goes to `005f20a0`
/// and the head is cleared (`005dd560`) when the face bit is in it.
///
pub fn fn_006099f0(e: &mut Engine, this: Ptr<TESNPC>, buffer: Ptr) {
    let base = this.addr();
    let mut changed: u8 = 0;
    e.call(0x009d_ace0, &args![this, buffer]);
    let mut follows_race = false;
    let race_height = tesnpc_get_race_height(e, this);
    if e.get(this, TESNPC::fHeight) == race_height {
        follows_race = true;
    }
    if change_flag_set(e, 0x0042_ce30, buffer, 0x200_0000)
        && !change_flag_set(e, 0x0042_8110, buffer, 0x200_0000)
        && !e.get(this, TESNPC::pOriginalRace).is_null()
    {
        let original = e.get(this, TESNPC::pOriginalRace);
        e.call(SET_WORD_AT_4, &args![base + COMPONENT_RACE, original]);
        e.set(this, TESNPC::pOriginalRace, Ptr::NULL);
        changed |= 0x1b;
    }
    if follows_race {
        let height = tesnpc_get_race_height(e, this);
        e.set(this, TESNPC::fHeight, height);
    }
    if change_flag_set(e, 0x0042_ce30, buffer, 0x800)
        && !change_flag_set(e, 0x0042_8110, buffer, 0x800)
    {
        let alternate = e.get(this, TESNPC::pAlternateFaceOffsetCoord);
        if !alternate.is_null() {
            e.call(0x005d_9ff0, &args![alternate, 3u32]);
        }
        e.set(this, TESNPC::pAlternateFaceOffsetCoord, Ptr::NULL);
        changed |= 8;
        e.call(0x0047_0470, &args![base + 0x1dc]);
    }
    if change_flag_set(e, 0x0042_ce30, buffer, 0x100_0000)
        && !change_flag_set(e, 0x0042_8110, buffer, 0x100_0000)
    {
        let bit = e.call(
            TEST_ACTOR_FLAGS,
            &args![base + COMPONENT_ACTOR_BASE_DATA, 1u32],
        );
        let cleared = bit.u8() == 0;
        e.call(
            0x0047_dd50,
            &args![base + COMPONENT_ACTOR_BASE_DATA, 1u32, cleared, 1u32],
        );
        changed |= 0x1b;
    }
    if changed & 8 != 0 {
        e.call(0x005d_d560, &args![this]);
    }
    if changed != 0 {
        e.call(0x005f_20a0, &args![this, changed as u32]);
    }
}

// Translated from 00609bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the NPC's record at `iFileOffset` can be found in `file`: the
/// file opens (`00470c70(0, 0)`), seeks to the offset (`004723a0`), the
/// record is of the NPC form type (the byte table at `011871f8`) and its
/// `008d8ac0` word is the NPC's form id.
pub fn fn_00609bf0(e: &mut Engine, this: Ptr<TESNPC>, file: Ptr) -> bool {
    if file.is_null() || e.get(this, TESNPC::iFileOffset) == 0 {
        return false;
    }
    if !e.call(0x0047_0c70, &args![file, 0u32, 0u32]).bool() {
        return false;
    }
    let offset = e.get(this, TESNPC::iFileOffset);
    if !e.call(0x0047_23a0, &args![file, offset]).bool() {
        return false;
    }
    let record_type = e.call(FILE_GET_FORM_TYPE, &args![file]).u32();
    if record_type != e.global::<u8>(0x0118_71f8) as u32 {
        return false;
    }
    let word = e.call(0x008d_8ac0, &args![file]).u32();
    word == e.call(GET_FORM_ID, &args![this]).u32()
}

// Translated from 00609c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size of the NPC's old-format face data: four bytes per element of the
/// 2 x 2 matrices (`width * height * 4` summed, as a `u16`), plus 21.
///
/// `_unused_1` is a word the exe never reads.
pub fn fn_00609c70(e: &mut Engine, this: Ptr<TESNPC>, _unused_1: u32) -> u16 {
    let base = this.addr();
    let mut size: u16 = 0;
    for row in 0..2u32 {
        for column in 0..2u32 {
            let matrix = base + 0x134 + row * 0x40 + column * 0x20;
            let width = e.call(0x0096_11e0, &args![matrix]).u32();
            let height = e.call(0x0044_1110, &args![matrix]).u32();
            size = (size as u32).wrapping_add(width.wrapping_mul(height).wrapping_mul(4)) as u16;
        }
    }
    for _ in 0..5 {
        size = size.wrapping_add(4);
    }
    size.wrapping_add(1)
}

// Translated from 00609d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the NPC's old-format face data with the old save calls: the
/// elements of the 2 x 2 matrices (`00484ce0`, four bytes each), the race's,
/// hair's and eyes' form ids (`00484d20`, 0 for none), the hair length and
/// colour, and one byte that is 1 for a female NPC (`005f0cc0` is 1).
///
/// `_unused_1` is a word the exe never reads.
pub fn fn_00609d60(e: &mut Engine, this: Ptr<TESNPC>, _unused_1: u32) {
    let base = this.addr();
    e.with_stack(0x14, |e, locals| {
        let iterator = locals.addr();
        let cell = locals.addr() + 8;
        for row in 0..2u32 {
            for column in 0..2u32 {
                let matrix = base + 0x134 + row * 0x40 + column * 0x20;
                let width = e.call(0x0096_11e0, &args![matrix]).u32();
                let height = e.call(0x0044_1110, &args![matrix]).u32();
                for x in 0..width {
                    for y in 0..height {
                        let at = e
                            .call(MATRIX_ITERATOR_AT, &args![matrix, iterator, x])
                            .u32();
                        let element = e.call(ITERATOR_ELEMENT, &args![at, y]).u32();
                        let word = e.mem.u32(element);
                        e.mem.set_u32(cell, word);
                        e.call(0x0048_4ce0, &args![this, cell, 4u32]);
                    }
                }
            }
        }
        let mut id = 0u32;
        if e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32() != 0 {
            let race = e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32();
            id = e.call(GET_FORM_ID, &args![race]).u32();
        }
        e.mem.set_u32(cell, id);
        e.call(0x0048_4d20, &args![this, cell, 4u32]);
        let mut id = 0u32;
        let hair = e.get(this, TESNPC::pHair);
        if !hair.is_null() {
            id = e.call(GET_FORM_ID, &args![hair]).u32();
        }
        e.mem.set_u32(cell, id);
        e.call(0x0048_4d20, &args![this, cell, 4u32]);
        let mut id = 0u32;
        let eyes = e.get(this, TESNPC::pEyeColor);
        if !eyes.is_null() {
            id = e.call(GET_FORM_ID, &args![eyes]).u32();
        }
        e.mem.set_u32(cell, id);
        e.call(0x0048_4d20, &args![this, cell, 4u32]);
        e.call(0x0048_4ce0, &args![this, base + 0x1bc, 4u32]);
        e.call(0x0048_4ce0, &args![this, base + 0x1d8, 4u32]);
        let female = e.call(GET_SEX, &args![this]).i32() == 1;
        e.mem.set_u8(cell, female as u8);
        e.call(0x0048_4ce0, &args![this, cell, 1u32]);
    });
}

// Translated from 00609f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::LoadFaceGen` (Xbox PDB): reads the NPC's face data written by
/// `fn_00609d60` (old save format) with the old load calls (`00484d00`,
/// `00484d40`), applies what differs from the NPC (matrix elements, race,
/// hair, eyes, hair length and colour, the sex bit), logging a missing form,
/// and, when anything changed, rebuilds the actor's head (`actor`): removes
/// the head nodes from the palette and the scene, clears the head
/// (`005dd560`), loads the face (`fn_00606820`) and applies the face-gen
/// parameters to both head nodes (`005dd590`, `006141f0`, `00655fa0`).
///
/// The exe's `race changed` flags (the stack bytes at `-0x31` and `-0x22`)
/// are set to 0 on every path, so its branch for a changed race (from
/// `0060a60c`, which reloads the 3D, picks a different model name and
/// restores first-person) can never run and is not translated.
pub fn fn_00609f60(e: &mut Engine, this: Ptr<TESNPC>, actor: Ptr) {
    const LOAD_BYTES: u32 = 0x0048_4d00;
    const LOAD_NUMERIC_ID: u32 = 0x0048_4d40;
    let base = this.addr();
    let mut changed = false;
    e.with_stack(0x10, |e, locals| {
        let value = locals.addr();
        let iterator = locals.addr() + 4;
        for row in 0..2u32 {
            for column in 0..2u32 {
                let matrix = base + 0x134 + row * 0x40 + column * 0x20;
                let width = e.call(0x0096_11e0, &args![matrix]).u32();
                let height = e.call(0x0044_1110, &args![matrix]).u32();
                for x in 0..width {
                    for y in 0..height {
                        e.call(LOAD_BYTES, &args![this, value, 4u32]);
                        let at = e
                            .call(MATRIX_ITERATOR_AT, &args![matrix, iterator, x])
                            .u32();
                        let element = e.call(ITERATOR_ELEMENT, &args![at, y]).u32();
                        let read = e.mem.f32(value);
                        if e.mem.f32(element) != read {
                            let at = e
                                .call(MATRIX_ITERATOR_AT, &args![matrix, iterator, x])
                                .u32();
                            let element = e.call(ITERATOR_ELEMENT, &args![at, y]).u32();
                            e.mem.set_f32(element, read);
                            changed = true;
                        }
                    }
                }
            }
        }
    });
    let name_component = base + COMPONENT_FULL_NAME;
    if e.call(NPC_GET_RACE, &args![this]).u32() == 0 {
        let name = e.call(0x0040_8da0, &args![name_component]).u32();
        e.call(LOG_MESSAGE, &args![0x0104_aa48u32, name]);
    }
    let race = e.with_stack(4, |e, id_cell| {
        e.call(LOAD_NUMERIC_ID, &args![this, id_cell, 4u32]);
        let id = e.mem.u32(id_cell.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let race = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_RACE, 0u32],
            )
            .u32();
        if id != 0 && race == 0 {
            let name = e.call(0x0040_8da0, &args![name_component]).u32();
            e.call(LOG_MESSAGE, &args![0x0104_aa20u32, name, id]);
        }
        race
    });
    if e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32() != race {
        changed = true;
    }
    e.call(SET_WORD_AT_4, &args![base + COMPONENT_RACE, race]);
    if e.call(NPC_GET_RACE, &args![this]).u32() == 0 {
        let name = e.call(0x0040_8da0, &args![name_component]).u32();
        e.call(LOG_MESSAGE, &args![0x0104_a9ecu32, name]);
    }
    let hair = e.with_stack(4, |e, id_cell| {
        e.call(LOAD_NUMERIC_ID, &args![this, id_cell, 4u32]);
        let id = e.mem.u32(id_cell.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let hair = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_HAIR, 0u32],
            )
            .u32();
        if id != 0 && hair == 0 {
            let name = e.call(0x0040_8da0, &args![name_component]).u32();
            e.call(LOG_MESSAGE, &args![0x0104_a9c4u32, name, id]);
        }
        hair
    });
    if e.get(this, TESNPC::pHair).addr() != hair {
        changed = true;
    }
    e.set(this, TESNPC::pHair, Ptr::new(hair));
    let eyes = e.with_stack(4, |e, id_cell| {
        e.call(LOAD_NUMERIC_ID, &args![this, id_cell, 4u32]);
        let id = e.mem.u32(id_cell.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let eyes = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_EYES, 0u32],
            )
            .u32();
        if id != 0 && eyes == 0 {
            let name = e.call(0x0040_8da0, &args![name_component]).u32();
            e.call(LOG_MESSAGE, &args![0x0104_a99cu32, name, id]);
        }
        eyes
    });
    if e.get(this, TESNPC::pEyeColor).addr() != eyes {
        changed = true;
    }
    e.set(this, TESNPC::pEyeColor, Ptr::new(eyes));
    let (length, color, sex_bit) = e.with_stack(0xc, |e, cells| {
        let (length, color, bit) = (cells.addr(), cells.addr() + 4, cells.addr() + 8);
        e.call(LOAD_BYTES, &args![this, length, 4u32]);
        e.call(LOAD_BYTES, &args![this, color, 4u32]);
        e.call(LOAD_BYTES, &args![this, bit, 1u32]);
        (e.mem.f32(length), e.mem.u32(color), e.mem.u8(bit))
    });
    if e.get(this, TESNPC::fHairLength) != length || e.get(this, TESNPC::iHairColor) != color {
        changed = true;
    }
    e.set(this, TESNPC::fHairLength, length);
    e.set(this, TESNPC::iHairColor, color);
    let current_bit = e
        .call(
            TEST_ACTOR_FLAGS,
            &args![base + COMPONENT_ACTOR_BASE_DATA, 1u32],
        )
        .u8();
    if current_bit != sex_bit {
        changed = true;
    }
    e.call(
        0x0047_dd50,
        &args![base + COMPONENT_ACTOR_BASE_DATA, 1u32, sex_bit as u32, 1u32],
    );
    if !changed {
        return;
    }
    let node = e.call(0x0043_fcd0, &args![actor]).u32();
    let mut root = 0;
    if node != 0 {
        root = e.vcall(node, 0xc, &args![]).u32();
    }
    // (The exe compares its two never-set race flags here and always takes
    // this side.)
    if root != 0 {
        let mut palette = 0;
        if e.call(0x008b_70d0, &args![actor]).u32() != 0 {
            let a = e.call(0x008b_70d0, &args![actor]).u32();
            if e.call(0x0049_6940, &args![a]).u32() != 0 {
                let a = e.call(0x008b_70d0, &args![actor]).u32();
                let b = e.call(0x0049_6940, &args![a]).u32();
                palette = e.call(0x0053_7bd0, &args![b]).u32();
            }
        }
        for slot in [0x1b0u32, 0x1ac] {
            let head = e.vcall(actor.addr(), slot, &args![0u32]).u32();
            if head != 0 && e.call(0x0096_11e0, &args![head]).u32() != 0 {
                e.call(0x00a6_e8e0, &args![head, palette]);
                let parent = e.call(0x0096_11e0, &args![head]).u32();
                e.vcall(parent, 0xe8, &args![head]);
            }
        }
    }
    e.call(0x005d_d560, &args![this]);
    let biped = e.call(0x005d_9f90, &args![actor]).u32();
    if biped != 0 {
        let biped = e.call(0x005d_9f90, &args![actor]).u32();
        tesnpc_linear_face_gen_head_load(e, this, actor, Ptr::new(biped));
    }
    if e.vcall(actor.addr(), 0x1ac, &args![0u32]).u32() != 0
        && e.vcall(actor.addr(), 0x1b0, &args![0u32]).u32() != 0
    {
        e.with_stack(FACE_PARAMS_SIZE, |e, params| {
            e.call(FACE_PARAMS_CONSTRUCT, &args![params]);
            let race = e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32();
            e.call(0x0061_41f0, &args![race, this, params, 0u32, 0u32]);
            let first = e.vcall(actor.addr(), 0x1ac, &args![0u32]).u32();
            e.call(APPLY_FACE_PARAMS, &args![first, params]);
            let second = e.vcall(actor.addr(), 0x1b0, &args![0u32]).u32();
            e.call(APPLY_FACE_PARAMS, &args![second, params]);
            e.call(FACE_PARAMS_DESTROY, &args![params]);
        });
    }
}

// Translated from 0060a890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the global flag at `011c5cb4` around the call of the actor's process'
/// virtual `0x464(actor)`. For the player it also takes the first-person biped
/// node (`00950bb0(player, 1)`) and its child `0045bc00(node, 0)`, and gives that
/// child the matrix `0056fac0(player, buffer, copy of 011a9448)` returns
/// (`0043fa80`).
///
/// `_this` is the `ECX` word the exe never reads.
///
pub fn fn_0060a890(e: &mut Engine, _this: Ptr<TESNPC>, actor: Ptr) {
    e.mem.set_u8(0x011c_5cb4, 1);
    let process = process_of(e, actor.addr());
    e.vcall(process, 0x464, &args![actor]);
    e.mem.set_u8(0x011c_5cb4, 0);
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if actor.addr() == player {
        let biped = e.call(0x0095_0bb0, &args![player, 1u32]).u32();
        let node = if biped != 0 {
            e.call(0x0045_bc00, &args![biped, 0u32]).u32()
        } else {
            0
        };
        // (The exe copies the 36-byte matrix at 011a9448 to a local it
        // never reads.)
        if node != 0 {
            e.with_stack(0x48, |e, frame| {
                let copy = frame.addr() + 0x24;
                let words = e.mem.bytes(0x011a_9448, 0x24);
                e.mem.write(copy, &words);
                let matrix = e.call(0x0056_fac0, &args![player, frame, copy]).u32();
                e.call(0x0043_fa80, &args![node, matrix]);
            });
        }
    }
}

// Translated from 0060a950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::SwapEyes` (Xbox PDB): finds the `"FaceGenEyeLeft"` and
/// `"FaceGenEyeRight"` children of `spHeadBiped` and gives both a new
/// texture, the eyes' own (`eyes + 0x24`, `"Data\Textures\%s"`) or the
/// default `Data\Textures\Characters\Eyes\EyeDefault.dds` for no `eyes`.
/// A shape that is usable (`0050d100`, and whose `0043b230` has a
/// `00441110` between 8 and 12) is retextured through its virtual `0xfc`;
/// otherwise a new `NiTexturingProperty` (`00a6aa40`, `0x30` bytes) with
/// the texture and clamp mode 3 is attached (property type 5 is replaced).
/// Finally the two eye nodes of the head are looked up again by name
/// (virtual `0x9c`) and prepared (`00b57e30`).
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn fn_0060a950(e: &mut Engine, this: Ptr<TESNPC>, eyes: Ptr) {
    let base = this.addr();
    // Layout: left eye pointer, right eye pointer, path string (8 bytes),
    // texture pointer, two fixed-string names.
    e.with_stack(0x24, |e, frame| {
        let left = frame.addr();
        let right = frame.addr() + 4;
        let path = frame.addr() + 8;
        let texture = frame.addr() + 0x10;
        let name_left = frame.addr() + 0x14;
        let name_right = frame.addr() + 0x18;
        e.call(NI_POINTER_CONSTRUCT, &args![left, 0u32]);
        e.call(NI_POINTER_CONSTRUCT, &args![right, 0u32]);
        if e.call(GET_FIRST_WORD, &args![base + 0x1c4]).u32() != 0 {
            let head = e.call(GET_FIRST_WORD, &args![base + 0x1c4]).u32();
            let count = e.call(0x0043_b480, &args![head]).u32();
            for index in 0..count {
                let head = e.call(GET_FIRST_WORD, &args![base + 0x1c4]).u32();
                let child = e.call(0x0043_b4a0, &args![head, index]).u32();
                for (eye_name, pointer) in [(0x0104_aac0u32, left), (0x0104_aab0u32, right)] {
                    let name = e.call(0x0041_3f40, &args![child]).u32();
                    let text = e.call(0x0043_b1b0, &args![name]).u32();
                    if e.call(STRCMP, &args![text, eye_name]).i32() == 0 {
                        let found = e.vcall(child, 0x1c, &args![]).u32();
                        e.call(NI_POINTER_ASSIGN, &args![pointer, found]);
                    }
                }
            }
            if e.call(GET_FIRST_WORD, &args![left]).u32() != 0
                && e.call(GET_FIRST_WORD, &args![right]).u32() != 0
            {
                e.call(0x0040_37b0, &args![path]);
                if !eyes.is_null() {
                    let texture_name = e.call(0x0040_8da0, &args![eyes.addr() + 0x24]).u32();
                    e.call(0x0040_6f60, &args![path, 0x0104_a64cu32, texture_name]);
                } else {
                    e.call(0x0040_6f60, &args![path, 0x0104_aa80u32]);
                }
                e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
                let text = e.call(GET_FIRST_WORD, &args![path]).u32();
                let tes = e.global::<u32>(TES_SINGLETON);
                e.call(0x0045_68c0, &args![tes, text, texture, 0u32, 0u32]);
                if e.call(GET_FIRST_WORD, &args![texture]).u32() != 0 {
                    let left_shape = e.call(GET_FIRST_WORD, &args![left]).u32();
                    let left_shape = e.call(0x0050_d100, &args![left_shape]).u32();
                    let right_shape = e.call(GET_FIRST_WORD, &args![right]).u32();
                    let right_shape = e.call(0x0050_d100, &args![right_shape]).u32();
                    if left_shape != 0 && right_shape != 0 {
                        let left_data = e.call(0x0043_b230, &args![left_shape]).u32();
                        // (The exe evaluates the size test twice for the
                        // left shape and keeps the second result.)
                        let _first = shape_size_in_range(e, left_data);
                        let left_usable = if shape_size_in_range(e, left_data) {
                            left_data
                        } else {
                            0
                        };
                        let right_data = e.call(0x0043_b230, &args![right_shape]).u32();
                        let right_usable = if shape_size_in_range(e, right_data) {
                            right_data
                        } else {
                            0
                        };
                        if left_usable != 0 && right_usable != 0 {
                            let held = e.call(GET_FIRST_WORD, &args![texture]).u32();
                            e.vcall(left_usable, 0xfc, &args![0u32, held]);
                            let held = e.call(GET_FIRST_WORD, &args![texture]).u32();
                            e.vcall(right_usable, 0xfc, &args![0u32, held]);
                            e.call(NI_POINTER_DESTROY, &args![texture]);
                            e.call(0x0040_37d0, &args![path]);
                            e.call(NI_POINTER_DESTROY, &args![right]);
                            e.call(NI_POINTER_DESTROY, &args![left]);
                            return;
                        }
                    }
                    let memory = e.call(0x00aa_13e0, &args![0x30u32]).u32();
                    let property = if memory != 0 {
                        e.call(0x00a6_aa40, &args![memory]).u32()
                    } else {
                        0
                    };
                    let held = e.call(GET_FIRST_WORD, &args![texture]).u32();
                    e.call(0x005b_8fc0, &args![property, held]);
                    e.call(0x004f_3200, &args![property, 3u32]);
                    fn_0060aeb0(e, Ptr::new(property), 2);
                    for pointer in [left, right] {
                        let node = e.call(GET_FIRST_WORD, &args![pointer]).u32();
                        if e.call(0x00a5_9d30, &args![node, 5u32]).u32() != 0 {
                            let node = e.call(GET_FIRST_WORD, &args![pointer]).u32();
                            e.call(0x00a5_b230, &args![node, 5u32]);
                        }
                        let node = e.call(GET_FIRST_WORD, &args![pointer]).u32();
                        e.call(0x0043_9410, &args![node, property]);
                    }
                }
                e.call(NI_POINTER_DESTROY, &args![texture]);
                e.call(0x0040_37d0, &args![path]);
            }
            let head = e.call(GET_FIRST_WORD, &args![base + 0x1c4]).u32();
            for (eye_name, fixed) in [(0x0104_aac0u32, name_left), (0x0104_aab0u32, name_right)] {
                let head = if eye_name == 0x0104_aac0 {
                    head
                } else {
                    e.call(GET_FIRST_WORD, &args![base + 0x1c4]).u32()
                };
                let name = e.call(0x0043_8170, &args![fixed, eye_name]).u32();
                let node = e.vcall(head, 0x9c, &args![name]).u32();
                e.call(0x0043_81b0, &args![fixed]);
                if node != 0 {
                    e.call(0x00b5_7e30, &args![node, 0u32, 0u32]);
                }
            }
        }
        e.call(NI_POINTER_DESTROY, &args![right]);
        e.call(NI_POINTER_DESTROY, &args![left]);
    });
}

/// `00441110(data)` between 8 and 12 for a non-null `data`.
fn shape_size_in_range(e: &mut Engine, data: u32) -> bool {
    if data == 0 {
        return false;
    }
    e.call(0x0044_1110, &args![data]).i32() >= 8 && {
        e.call(0x0044_1110, &args![data]).i32() <= 0xc
    }
}

// Translated from 0060aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes sure the object at `this + 0x1c` has its first slot filled
/// (`00877a30(0)` returns the slot): an empty slot gets a new 16-byte object
/// (`00a69dd0`) stored with `0096ae90(0, &object)`. Then
/// `fn_0060af60(object, argument)`.
pub fn fn_0060aeb0(e: &mut Engine, this: Ptr, argument: u32) {
    let owner = this.addr() + 0x1c;
    e.with_stack(4, |e, cell| {
        let slot = e.call(0x0087_7a30, &args![owner, 0u32]).u32();
        let mut object = e.mem.u32(slot);
        e.mem.set_u32(cell.addr(), object);
        if object == 0 {
            let memory = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
            object = if memory != 0 {
                e.call(0x00a6_9dd0, &args![memory]).u32()
            } else {
                0
            };
            e.mem.set_u32(cell.addr(), object);
            e.call(0x0096_ae90, &args![owner, 0u32, cell]);
        }
        let object = e.mem.u32(cell.addr());
        fn_0060af60(e, Ptr::new(object), argument as u16);
    });
}

// Translated from 0060af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004f32e0(this, argument, 0xf00, 8)`.
pub fn fn_0060af60(e: &mut Engine, this: Ptr, argument: u16) {
    e.call(0x004f_32e0, &args![this, argument as u32, 0xf00u32, 8u32]);
}

// Translated from 0060af90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::BuildDefaultModelList` (Xbox PDB): makes a new `BSSimpleList` (8
/// bytes, constructed by `0096a2d0`) and fills it. With `biped_models`, for each
/// of the 20 biped slots whose item `004829c0(this, slot)` exists, the slot's
/// biped model (`item + 0x70`, then `004811e0(model, sex)`) is appended
/// (`005ae3d0`), skipping a slot whose model `00480af0(slot 2's model, slot, 0, 0)`
/// reports as covered, and slots already seen; with `extra_model`, the model that
/// the object at `00482910(this)`'s `+0x3c` gives through its virtual `0x14` is
/// appended too. Returns the list.
///
/// Not translated: the compiler's exception-unwinding frame.
///
pub fn tesnpc_build_default_model_list(
    e: &mut Engine,
    this: Ptr<TESNPC>,
    biped_models: u8,
    extra_model: u8,
) -> Ptr {
    let base = this.addr();
    let memory = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let list = if memory != 0 {
        e.call(0x0096_a2d0, &args![memory]).u32()
    } else {
        0
    };
    if biped_models != 0 {
        let mut slot_two = 0u32;
        let sex = e.call(GET_SEX, &args![this]).u32();
        e.with_stack(0x58, |e, frame| {
            let models = frame.addr();
            let cell = frame.addr() + 0x50;
            e.call(MEMSET, &args![models, 0u32, 0x50u32]);
            for slot in 0..0x14i32 {
                let item = e
                    .call(0x0048_29c0, &args![base + COMPONENT_CONTAINER, this, slot])
                    .u32();
                if item == 0 {
                    continue;
                }
                if slot_two != 0
                    && e.call(0x0048_0af0, &args![slot_two, slot, 0u32, 0u32])
                        .bool()
                {
                    continue;
                }
                let entry = models + slot as u32 * 4;
                if e.mem.u32(entry) != 0 {
                    continue;
                }
                e.mem
                    .set_u32(entry, if item != 0 { item + 0x70 } else { 0 });
                if e.mem.u32(entry) != 0 {
                    let model = e.mem.u32(entry);
                    let found = e.call(0x0048_11e0, &args![model, sex]).u32();
                    e.mem.set_u32(cell, found);
                    if found != 0 {
                        e.call(0x005a_e3d0, &args![list, cell]);
                    }
                }
                if slot == 2 {
                    slot_two = e.mem.u32(entry);
                }
            }
        });
    }
    if extra_model != 0 {
        let source = e
            .call(0x0048_2910, &args![base + COMPONENT_CONTAINER, this])
            .u32();
        if source != 0 {
            let object = if source != 0 { source + 0x3c } else { 0 };
            if object != 0 {
                let model = e.vcall(object, 0x14, &args![]).u32();
                if model != 0 {
                    e.with_stack(4, |e, cell| {
                        e.mem.set_u32(cell.addr(), model);
                        e.call(0x005a_e3d0, &args![list, cell]);
                    });
                }
            }
        }
    }
    Ptr::new(list)
}

// Translated from 0060b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00461580(this, 0x800)`: the `0x800` bit of an actor-base flags
/// component.
pub fn fn_0060b1d0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(TEST_ACTOR_FLAGS, &args![this, 0x800u32]).u32()
}

// Translated from 0060b1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00461580(this, 0x1000)`: the `0x1000` bit of an actor-base flags
/// component.
pub fn fn_0060b1f0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(TEST_ACTOR_FLAGS, &args![this, 0x1000u32]).u32()
}

// Translated from 00607990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::Activate` (Xbox PDB): what happens when `activator_ref` activates the
/// NPC reference `target_ref` (the form's `Activate` virtual). Returns true when
/// the activation was handled, false when it was refused.
///
/// The exe's function is a long chain of tests on the two actors; its code is
/// followed block by block, and each block that more than one path reaches is a
/// method of [`Activation`] named after its address. In outline: nothing happens
/// for a target that is not an actor, is refused by its virtual `0x2e8`, has no
/// process or whose process' virtual `0x610` answers; some states of the target
/// (virtual `0x22c`, `0x230`, kind `004f8960` = 6, fleeing `008a6650`) show a
/// pop-up (`007052f0`); the companion menu, the VATS menu and the other menus
/// are queued with `00709470` (type 8, 4 or 1); a process' virtual `0x33c` lets
/// an NPC activator act on the target; the dialogue topic and item come from
/// `0061a2d0`, `0061b320` and are started with `0057b7c0`; `item` (`count` of
/// them) is handed over through the target's virtual `0x17c`; the processes'
/// virtual `0x288` sets the two actors' package state.
///
/// `_unused_1` is a word the exe never reads.
///
pub fn tesnpc_activate(
    e: &mut Engine,
    this: Ptr<TESNPC>,
    target_ref: Ptr,
    activator_ref: Ptr,
    _unused_1: u32,
    item: Ptr,
    count: u32,
) -> bool {
    e.with_stack(0x138, |e, frame| {
        let activator = e
            .call(
                RT_DYNAMIC_CAST,
                &args![activator_ref, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
            )
            .u32();
        let target = e
            .call(
                RT_DYNAMIC_CAST,
                &args![target_ref, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
            )
            .u32();
        if target == 0 || e.vcall(target, 0x2e8, &args![]).bool() {
            return false;
        }
        let target_process = process_of(e, target);
        if target_process == 0 {
            return false;
        }
        if e.vcall(target_process, 0x52c, &args![]).u32() != 0
            && !e.vcall(target, 0x22c, &args![0u32]).bool()
        {
            return false;
        }
        if e.vcall(target_process, 0x610, &args![]).u32() != 0 {
            return false;
        }
        let activator_process = process_of(e, activator);
        let activator_state = e.vcall(activator_process, 0x27c, &args![]).u32();
        let target_state = e.vcall(target_process, 0x27c, &args![]).u32();
        let activation = Activation {
            this,
            target_ref: target_ref.addr(),
            activator_ref: activator_ref.addr(),
            item: item.addr(),
            count,
            activator,
            target,
            target_process,
            activator_state,
            target_state,
            buffer: frame.addr(),
            flag: frame.addr() + 0x130,
        };
        activation.start(e)
    })
}

/// The calendar object (`011de7b8`) the hour (`00867da0`) and day
/// (`00867d60`) getters take as `this`.
const CALENDAR: u32 = 0x011d_e7b8;
/// `sprintf(buffer, format, ...)` of the C runtime (cdecl).
const SPRINTF: u32 = 0x00ec_623a;
/// `"%s %s"`.
const FORMAT_TWO_STRINGS: u32 = 0x0101_2058;
/// `Interface::QueueMenuCreate(type, reference, ...)` (cdecl, six words).
const QUEUE_MENU_CREATE: u32 = 0x0070_9470;
/// `VATS::QuitVATSPlayback` (thiscall on `011f2250`, two words).
const VATS_QUIT_PLAYBACK: u32 = 0x009c_8950;
const VATS_OBJECT: u32 = 0x011f_2250;
/// The `float` `2.0` the message display time is read from.
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// The message icons: `glow_message_vaultboy_surprised.dds` (the path in
/// the exe is misspelled `Interfac\`), `..._sad.dds`.
const ICON_SURPRISED: u32 = 0x0104_a958;
const ICON_SAD: u32 = 0x0102_08a0;

/// Who the first `%s` of a message names.
enum Speaker {
    /// The activated actor: `0055d520(target)`.
    Target,
    /// The NPC form itself: its full name (`00408da0(this + 0xd0)`).
    Npc,
}

/// The state of one `TESNPC::Activate` call.
struct Activation {
    this: Ptr<TESNPC>,
    /// The two reference arguments as passed (before the casts).
    target_ref: u32,
    activator_ref: u32,
    item: u32,
    count: u32,
    /// The casts of the two references to `Actor` (`-0x14` and `-0x8`).
    activator: u32,
    target: u32,
    /// The target's process (`-0x4`) and the words the processes' virtual
    /// `0x27c` returned for the activator (`-0x10`) and the target (`-0xc`).
    target_process: u32,
    activator_state: u32,
    target_state: u32,
    /// The message text buffer (`0x130` bytes) and a word the actor tests
    /// (`008b06d0`) write to.
    buffer: u32,
    flag: u32,
}

impl Activation {
    fn player(e: &Engine) -> u32 {
        e.global::<u32>(PLAYER_SINGLETON)
    }

    /// `MobileObject::GetCurrentPackage` (`009344a0`).
    fn package(e: &mut Engine, actor: u32) -> u32 {
        e.call(0x0093_44a0, &args![actor]).u32()
    }

    /// The pop-up message: the text of the setting at `setting`
    /// (`00403df0`), the speaker's name, `"%s %s"`, then the display call
    /// `007052f0(text, 0, icon, 0, 2.0, 0)`.
    fn message(&self, e: &mut Engine, setting: u32, speaker: Speaker, icon: u32) {
        let text = e.call(0x0040_3df0, &args![setting]).u32();
        let name = match speaker {
            Speaker::Target => e.call(0x0055_d520, &args![self.target]).u32(),
            Speaker::Npc => e
                .call(0x0040_8da0, &args![self.this.addr() + COMPONENT_FULL_NAME])
                .u32(),
        };
        e.call(SPRINTF, &args![self.buffer, FORMAT_TWO_STRINGS, name, text]);
        let time = e.mem.u32(MESSAGE_TIME);
        e.call(
            0x0070_52f0,
            &args![self.buffer, 0u32, icon, 0u32, time, 0u32],
        );
    }

    /// `QueueMenuCreate(menu, reference, a, b, c, 0)`.
    fn queue_menu(&self, e: &mut Engine, menu: u32, reference: u32, a: u32, b: u32, c: u32) {
        e.call(QUEUE_MENU_CREATE, &args![menu, reference, a, b, c, 0u32]);
    }

    /// 00607ab4: the player's own activation of a downed actor, then the
    /// refusal messages for the activated actor's state.
    fn start(&self, e: &mut Engine) -> bool {
        let player = Self::player(e);
        let t = self.target;
        if t == player && e.mem.u8(player + 0x20c) != 0 {
            if e.vcall(t, 0x304, &args![]).bool() && e.call(0x008a_61b0, &args![t]).bool() {
                let hour = e.call(0x0086_7da0, &args![CALENDAR]).f32();
                let one = e.global::<f64>(ONE_DOUBLE);
                let earlier = (f64::from(hour) - one) as f32;
                e.call(0x0069_3d50, &args![self.target_process, earlier]);
            }
            return false;
        }
        if e.vcall(t, 0x22c, &args![0u32]).bool() && e.call(0x004f_8960, &args![t]).u32() != 6 {
            return self.at_00607c8b(e);
        }
        if e.call(0x0043_7bd0, &args![t]).bool()
            || e.vcall(t, 0x230, &args![]).bool()
            || e.call(0x004f_8960, &args![t]).u32() == 6
        {
            self.message(e, 0x011d_210c, Speaker::Target, ICON_SURPRISED);
            return false;
        }
        if !e.call(0x008a_6650, &args![t, 0u32]).bool() {
            return self.at_00607c8b(e);
        }
        let combat_target = e.vcall(t, 0x428, &args![]).u32();
        if combat_target != 0
            && e.call(0x0097_fa10, &args![combat_target, player]).bool()
            && !e.call(0x0089_4d60, &args![player]).bool()
        {
            return self.at_00607c8b(e);
        }
        self.message(e, 0x011d_2538, Speaker::Target, ICON_SAD);
        false
    }

    /// 00607c8b: the companion menu, then the package extra data handling.
    fn at_00607c8b(&self, e: &mut Engine) -> bool {
        let player = Self::player(e);
        let (a, t) = (self.activator, self.target);
        if e.call(0x0056_6950, &args![t]).bool() && self.activator_ref == player {
            if e.call(0x0075_4d90, &args![t]).bool() {
                self.queue_menu(e, 8, self.target_ref, 0, 0, 0);
            }
            return false;
        }
        if a == 0 {
            return false;
        }
        let owner = e.call(0x005d_43c0, &args![a]).u32();
        let package_extra = e.call(0x0041_cb10, &args![owner]).u32();
        if a != player
            && package_extra != 0
            && e.call(0x0041_ca90, &args![package_extra]).u32() == 0xf
        {
            let target_extras = e.call(0x005d_43c0, &args![t]).u32();
            let package_target = e.call(0x0041_cb70, &args![target_extras]).u32();
            let extras = e.call(0x005d_43c0, &args![a]).u32();
            e.call(
                0x0041_c930,
                &args![
                    extras,
                    package_extra,
                    4u32,
                    package_target,
                    1u32,
                    1u32,
                    0u32
                ],
            );
            if e.call(0x0067_0f90, &args![package_extra]).bool() {
                let day = e.call(0x0086_7d60, &args![CALENDAR]).u8();
                e.vcall(a, 0x28c, &args![package_extra, day as u32]);
            }
            let process = process_of(e, t);
            e.vcall(process, 0x5a0, &args![a, package_extra]);
            e.vcall(t, 0x48, &args![0x8000_0000u32]);
        }
        if a != player && process_of(e, a) != 0 {
            let process = process_of(e, a);
            e.vcall(process, 0x4ec, &args![1u32]);
        }
        if e.call(0x0049_3bb0, &args![t]).bool()
            && a == player
            && e.call(0x0056_6950, &args![t]).bool()
        {
            e.call(VATS_QUIT_PLAYBACK, &args![VATS_OBJECT, 0u32, 0u32]);
            self.queue_menu(e, 4, t, 0, 0, 1);
            return true;
        }
        if e.call(0x0049_3bb0, &args![t]).bool() && !e.vcall(t, 0x230, &args![]).bool() {
            return self.at_00608cbf(e);
        }
        if a == player
            && e.vcall(t, 0x214, &args![]).u32() != 0
            && e.vcall(t, 0x214, &args![]).u32() != 9
            && e.vcall(t, 0x214, &args![]).u32() != 4
        {
            let process = process_of(e, t);
            let record = e.vcall(process, 0x4d4, &args![]).u32();
            if record == 0 {
                return self.at_00608cbf(e);
            }
            let process = process_of(e, t);
            let record = e.vcall(process, 0x4d4, &args![]).u32();
            if e.mem.u8(record + 0xe) <= 0x13 {
                return self.at_00608cbf(e);
            }
        }
        self.at_00607f45(e)
    }

    /// 00607f45: attacks on a defenceless target.
    fn at_00607f45(&self, e: &mut Engine) -> bool {
        let player = Self::player(e);
        let (a, t) = (self.activator, self.target);
        if fn_00608d80(e, Ptr::new(t)) {
            return false;
        }
        if e.vcall(t, 0x22c, &args![0u32]).bool() {
            return self.at_00608c3e(e);
        }
        e.mem.set_u32(self.flag, 0);
        let skip = e.call(0x0043_7bf0, &args![t]).bool() || e.vcall(t, 0x234, &args![]).bool();
        if !skip {
            if a != player
                && e.call(0x008b_06d0, &args![a, t, 0u32, self.flag, 0u32])
                    .bool()
            {
                let process = process_of(e, a);
                if e.vcall(
                    process,
                    0x33c,
                    &args![a, t, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32],
                )
                .bool()
                {
                    return true;
                }
                return self.at_006080c2(e);
            }
            if !e.vcall(t, 0x304, &args![]).bool()
                && e.call(0x008b_06d0, &args![t, a, 0u32, self.flag, 0u32])
                    .bool()
                && !e.call(0x0049_97b0, &args![a]).bool()
            {
                let process = process_of(e, t);
                if e.vcall(
                    process,
                    0x33c,
                    &args![t, a, 1u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32],
                )
                .bool()
                {
                    return true;
                }
            }
        }
        self.at_006080c2(e)
    }

    /// 006080c2: the player cannot talk to an essential/dead target.
    fn at_006080c2(&self, e: &mut Engine) -> bool {
        let player = Self::player(e);
        let (a, t) = (self.activator, self.target);
        if a == player
            && e.vcall(t, 0x230, &args![]).bool()
            && e.call(0x0087_f3d0, &args![t]).bool()
        {
            return false;
        }
        if a != player {
            if t != player {
                return self.at_00608937(e);
            }
            let package = Self::package(e, a);
            if !e.call(0x0067_8610, &args![package]).bool() {
                let package = Self::package(e, a);
                if e.call(0x0041_ca90, &args![package]).u32() != 0 {
                    let package = Self::package(e, a);
                    if e.call(0x0041_ca90, &args![package]).u32() != 9 {
                        return self.at_00608937(e);
                    }
                }
            }
        }
        self.at_0060815a(e)
    }

    /// 0060815a: the player's refusals before a dialogue.
    fn at_0060815a(&self, e: &mut Engine) -> bool {
        let player = Self::player(e);
        let (a, t) = (self.activator, self.target);
        if a != player {
            return self.at_00608272(e);
        }
        if !e.vcall(t, 0x230, &args![]).bool() {
            if !e.call(0x0049_97b0, &args![player]).bool() {
                return self.at_00608272(e);
            }
            if e.call(0x0056_6950, &args![t]).bool() {
                return self.at_00608272(e);
            }
        }
        if e.call(0x008a_ce90, &args![t]).bool() {
            return self.at_00608272(e);
        }
        if !e.vcall(t, 0x230, &args![]).bool() {
            let process = process_of(e, t);
            if e.vcall(process, 0x110, &args![]).bool() {
                self.message(e, 0x011d_2394, Speaker::Target, ICON_SAD);
                return false;
            }
        }
        self.queue_menu(e, 1, self.target_ref, 0, 0, 2);
        true
    }

    /// 00608272: finds the dialogue topic and its first item.
    fn at_00608272(&self, e: &mut Engine) -> bool {
        let player = Self::player(e);
        let (a, t) = (self.activator, self.target);
        let mut topic = 0;
        let mut wants_topic = a == player;
        if !wants_topic {
            let process = process_of(e, a);
            wants_topic = e.vcall(process, 0x604, &args![]).bool();
        }
        if wants_topic {
            topic = e.call(0x0061_a2d0, &args![0u32, 0u32]).u32();
        }
        let mut dialogue = 0;
        if topic != 0 {
            if a == player {
                dialogue = e
                    .call(0x0061_b320, &args![topic, t, player, 0u32, 0u32, 0u32])
                    .u32();
            } else {
                let process = process_of(e, a);
                if e.vcall(process, 0x604, &args![]).bool() {
                    dialogue = e
                        .call(0x0061_b320, &args![topic, a, player, 0u32, 0u32, 0u32])
                        .u32();
                }
            }
        }
        if dialogue != 0 {
            let id = e.call(GET_FORM_ID, &args![dialogue]).u32();
            if e.call(0x0061_9df0, &args![id]).bool()
                && e.call(0x0083_c7b0, &args![dialogue]).bool()
                && !e.call(0x0083_c7e0, &args![dialogue]).bool()
            {
                if e.vcall(t, 0x214, &args![]).u32() == 9 {
                    let process = process_of(e, t);
                    e.vcall(process, 0x600, &args![1u32]);
                    e.vcall(t, 0x418, &args![]);
                    return true;
                }
                // 0060840b / 00608420: the word pushed is the topic (`[EBP-0x160]`, from
                // 0061a2d0), not the dialogue item (`[EBP-0x15c]`).
                if t == player {
                    e.call(0x0057_b7c0, &args![a, topic, 0u32, 0u32]);
                } else {
                    e.call(0x0057_b7c0, &args![t, topic, 0u32, 0u32]);
                }
                return self.finish_dialogue(e, dialogue);
            }
        }
        self.at_00608434(e, dialogue)
    }

    /// 006088f2: frees the dialogue item; the activation counts as handled.
    fn finish_dialogue(&self, e: &mut Engine, dialogue: u32) -> bool {
        if dialogue != 0 {
            e.call(0x005c_90d0, &args![dialogue, 1u32]);
        }
        true
    }

    /// 00608434: starts a conversation (the camera and process side).
    fn at_00608434(&self, e: &mut Engine, dialogue: u32) -> bool {
        let player = Self::player(e);
        let (a, t, tp) = (self.activator, self.target, self.target_process);
        let process = process_of(e, t);
        if e.vcall(process, 0x4d4, &args![]).u32() != 0 {
            let process = process_of(e, t);
            let record = e.vcall(process, 0x4d4, &args![]).u32();
            if e.mem.u8(record + 0xe) > 0x13 {
                let process = process_of(e, t);
                e.vcall(process, 0x614, &args![0x200u32]);
            }
        }
        if e.vcall(t, 0x214, &args![]).u32() == 0 {
            face_actor_towards(e, a, t);
        }
        let mut menu_data = 0u32;
        if a == player {
            if !e.call(0x008b_3bb0, &args![t]).bool() && !e.call(0x008b_3c30, &args![t]).bool() {
                let process = process_of(e, t);
                e.vcall(process, 0x294, &args![t]);
            }
            e.call(0x008a_8e50, &args![a]);
            e.call(0x008a_8e50, &args![t]);
            let package = Self::package(e, a);
            let cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![package, 0u32, TYPE_TES_PACKAGE, TYPE_DIALOGUE_PACKAGE, 0u32],
                )
                .u32();
            if cast != 0 {
                menu_data = e.mem.u32(cast + 0x8c);
            }
            if e.vcall(t, 0x2c8, &args![]).u32() == player
                && self.target_state != 0
                && e.call(0x0041_ca90, &args![self.target_state]).u32() == 0
            {
                let mut reached = false;
                if e.call(0x0041_d8a0, &args![self.target_state]).u32() != 0 {
                    let list = e.call(0x0041_d8a0, &args![self.target_state]).u32();
                    if e.call(GET_WORD_AT_4, &args![list]).u32() == 0 {
                        e.vcall(tp, 0x288, &args![a, 2u32]);
                        reached = true;
                    }
                }
                if !reached {
                    e.vcall(tp, 0x288, &args![t, 1u32]);
                }
            }
            let audio = e.call(0x0045_3a70, &args![]).u32();
            e.call(0x00ad_8780, &args![audio, 4u32]);
            e.call(0x0081_5b00, &args![t + 0x88]);
            if e.vcall(t, 0x214, &args![]).u32() != 9 {
                let mut subject = a;
                if subject == player {
                    subject = t;
                }
                e.call(VATS_QUIT_PLAYBACK, &args![VATS_OBJECT, 0u32, 0u32]);
                self.queue_menu(e, 4, subject, menu_data, 0, 1);
            } else {
                let process = process_of(e, t);
                e.vcall(process, 0x600, &args![1u32]);
                e.vcall(t, 0x418, &args![]);
            }
        } else if e.call(0x0070_2640, &args![]).u32() != 0x3f1 {
            let player_process = process_of(e, player);
            if !e.vcall(player_process, 0x3fc, &args![a]).bool() {
                return false;
            }
            let audio = e.call(0x0045_3a70, &args![]).u32();
            e.call(0x00ad_8780, &args![audio, 4u32]);
            e.call(0x008a_8e50, &args![a]);
            e.call(0x008a_8e50, &args![t]);
            if !e.call(0x0049_3bb0, &args![a]).bool() {
                let package = Self::package(e, a);
                let cast = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![package, 0u32, TYPE_TES_PACKAGE, TYPE_DIALOGUE_PACKAGE, 0u32],
                    )
                    .u32();
                if cast != 0 {
                    menu_data = e.mem.u32(cast + 0x8c);
                }
            }
            let mut set_one = true;
            if e.call(0x0041_d8a0, &args![self.activator_state]).u32() != 0 {
                let list = e.call(0x0041_d8a0, &args![self.activator_state]).u32();
                if e.call(GET_WORD_AT_4, &args![list]).u32() == 0 {
                    let process = process_of(e, a);
                    e.vcall(process, 0x288, &args![a, 2u32]);
                    set_one = false;
                }
            }
            if set_one {
                let process = process_of(e, a);
                e.vcall(process, 0x288, &args![a, 1u32]);
            }
            e.call(0x0081_5b00, &args![t + 0x88]);
            if t == player {
                e.call(0x008a_7a90, &args![t]);
            } else {
                let process = process_of(e, t);
                e.vcall(process, 0x614, &args![0x400u32]);
            }
            if a == player {
                e.call(0x008a_7a90, &args![a]);
            } else {
                let process = process_of(e, a);
                e.vcall(process, 0x614, &args![0x400u32]);
            }
            e.call(VATS_QUIT_PLAYBACK, &args![VATS_OBJECT, 0u32, 0u32]);
            let mut subject = a;
            if subject == player {
                subject = t;
            }
            self.queue_menu(e, 4, subject, menu_data, 0, 1);
        }
        self.finish_dialogue(e, dialogue)
    }

    /// 00608937: the activator is not the player and the target is neither
    /// a dialogue partner nor a downed player: item hand-over and package
    /// changes.
    fn at_00608937(&self, e: &mut Engine) -> bool {
        let player = Self::player(e);
        let (a, t, tp) = (self.activator, self.target, self.target_process);
        if self.item != 0 {
            let npc_item = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![self.item, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_NPC, 0u32],
                )
                .u32();
            if npc_item == 0 {
                e.vcall(
                    t,
                    0x17c,
                    &args![self.item, 0u32, self.count, 1u32, 0u32, a, 0u32, 0u32, 1u32, 0u32],
                );
                e.call(0x008c_00e0, &args![a, t, self.item, self.count]);
                return true;
            }
        }
        let package = Self::package(e, a);
        if e.call(0x0041_ca90, &args![package]).u32() == 2 {
            let process = process_of(e, a);
            e.vcall(process, 0x288, &args![a, 2u32]);
            return true;
        }
        let package = Self::package(e, a);
        if package != 0 {
            let package = Self::package(e, a);
            if e.call(0x0041_ca90, &args![package]).u32() != 1 {
                let process = process_of(e, a);
                e.vcall(process, 0x288, &args![a, 1u32]);
            }
        }
        if t == player {
            return true;
        }
        if t != a {
            let package = Self::package(e, t);
            if package != 0 {
                let package = Self::package(e, t);
                if e.call(0x0041_ca90, &args![package]).u32() != 1 {
                    let package = Self::package(e, t);
                    if e.call(0x0041_ca90, &args![package]).u32() != 2 {
                        e.vcall(tp, 0x288, &args![t, 1u32]);
                    }
                }
            }
        }
        if t != a && e.vcall(t, 0x214, &args![]).u32() == 0 {
            if e.call(0x0093_36c0, &args![a]).bool() {
                let package = Self::package(e, a);
                if !e.call(0x0067_2800, &args![package]).bool()
                    && !e.call(0x0067_27b0, &args![package]).bool()
                {
                    face_actor_towards(e, a, t);
                }
            } else {
                face_actor_towards(e, a, t);
            }
        }
        if !e.call(0x0093_36c0, &args![a]).bool()
            && e.vcall(a, 0x218, &args![]).bool()
            && e.vcall(t, 0x218, &args![]).bool()
        {
            let process = process_of(e, a);
            e.vcall(process, 0x288, &args![a, 1u32]);
            if e.vcall(
                a,
                0x280,
                &args![t, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .bool()
            {
                let process = process_of(e, a);
                e.vcall(process, 0x288, &args![a, 2u32]);
                e.vcall(tp, 0x288, &args![t, 2u32]);
            }
        }
        true
    }

    /// 00608c3e: the target is down or asleep: the player gets its menu,
    /// another actor hands over the item.
    fn at_00608c3e(&self, e: &mut Engine) -> bool {
        let player = Self::player(e);
        let a = self.activator;
        if a == player {
            self.queue_menu(e, 1, self.target_ref, 0, 0, 1);
            return true;
        }
        if self.item != 0 {
            let npc_item = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![self.item, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_NPC, 0u32],
                )
                .u32();
            if npc_item == 0 {
                e.vcall(
                    self.target_ref,
                    0x17c,
                    &args![self.item, 0u32, self.count, 0u32, 0u32, a, 0u32, 0u32, 1u32, 0u32],
                );
            }
        }
        true
    }

    /// 00608cbf: the target's combat target tells the player off.
    fn at_00608cbf(&self, e: &mut Engine) -> bool {
        let player = Self::player(e);
        let (a, t) = (self.activator, self.target);
        if t != 0 && e.vcall(t, 0x428, &args![]).u32() != 0 && a == player {
            let combat_target = e.vcall(t, 0x428, &args![]).u32();
            if e.call(0x0047_c850, &args![combat_target]).bool() {
                self.message(e, 0x011c_f594, Speaker::Npc, 0);
                return false;
            }
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Session 3: the tail of the unit (0060b210 to 0060bed0)
//
// The functions from `0060b340` on are the compiler's instances of the
// `FR2MatrixVTC<float>` (a `std::vector<float>` plus rows and columns) and of
// the Gamebryo / Bethesda arrays the NPC owns, in the order the exe emitted
// them. Every wrapper that takes the address of its own argument
// (`0065fe40(&arg)` returns the word the pointer points to) keeps its words in
// game memory (`with_argument_words`); the "tag" words the exe pushes for
// overload selection (`0065e750` returns an uninitialised stack byte) are
// passed on as the exe passes them and never read by anyone.

/// `std::vector` iterator (`FR2MatrixVTC<float>`'s `iterator`, 8 bytes): the
/// word at `+0` is the container proxy (a pointer to a word holding the
/// container), the word at `+4` the element pointer.
/// `GetSize` of the matrix vector (`00662820`, thiscall): `(last - first) >> 2`.
const VECTOR_SIZE: u32 = 0x0066_2820;
/// `max_size` of the matrix vector (`0044a2c0`, thiscall).
const VECTOR_MAX_SIZE: u32 = 0x0044_a2c0;
/// `allocate(count)` of the vector's allocator (`0044abd0`, thiscall on the
/// allocator at `this + 8`).
const VECTOR_ALLOCATE: u32 = 0x0044_abd0;
/// `_Destroy(first, last)` (`0065f2b0`, thiscall on the vector).
const VECTOR_DESTROY: u32 = 0x0065_f2b0;
/// `deallocate(pointer, count)` of the vector's allocator (`0064df10`).
const VECTOR_DEALLOCATE: u32 = 0x0064_df10;
/// `_Xlen`: throws `length_error` (`0065f640`).
const VECTOR_LENGTH_ERROR: u32 = 0x0065_f640;
/// `begin(out)` (`0044a170`, thiscall) and `end(out)` (`0065f550`).
const VECTOR_BEGIN: u32 = 0x0044_a170;
const VECTOR_END: u32 = 0x0065_f550;
/// `erase(out, first.proxy, first.pointer, last.proxy, last.pointer)`
/// (`0044a1a0`, thiscall, five words).
const VECTOR_ERASE: u32 = 0x0044_a1a0;
/// Checked access `&element[index]` (`006578b0`, thiscall).
const VECTOR_ELEMENT_ADDRESS: u32 = 0x0065_78b0;
/// `_invalid_parameter` (`00ec7c56`), the checked iterators' failure.
const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// Overload-selection tag function (`0065e750`): returns an uninitialised byte.
const OVERLOAD_TAG: u32 = 0x0065_e750;
/// Returns its first argument (`0065fd70`).
const RETURN_FIRST_ARGUMENT: u32 = 0x0065_fd70;
/// Returns the word its argument points to (`0065fe40`, unchecked iterator).
const UNWRAP_ITERATOR: u32 = 0x0065_fe40;
/// `_Uninitialized_copy(first, last, destination, allocator, tag, tag)` (`0060c270`).
const UNINITIALIZED_COPY: u32 = 0x0060_c270;
/// `_Copy_backward(first, last, destinationEnd, ...)` (`0060c290`).
const COPY_BACKWARD: u32 = 0x0060_c290;
/// `_Uninitialized_fill_n(first, count, valuePointer)` (`0060c2e0`).
const UNINITIALIZED_FILL: u32 = 0x0060_c2e0;
/// `NiTPrimitiveArray<FaceGenUndo *>::SetSize`-like growth (`0060bef0`).
const ARRAY_GROW: u32 = 0x0060_bef0;
/// Store at an index of the `NiTArray` (`0060c120`).
const ARRAY_STORE: u32 = 0x0060_c120;
/// Frees a block (`00401030`, cdecl).
const FREE_BLOCK: u32 = 0x0040_1030;
/// The memory manager singleton getter (`00401020`) and
/// `MemoryManager::GetThreadScrapHeap` (`00aa42e0`, Xbox PDB).
const MEMORY_MANAGER: u32 = 0x0040_1020;
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;

/// Vtables of the arrays the NPC uses (RTTI: `NiTArray<FaceGenUndo *>`,
/// `NiTPrimitiveArray<FaceGenUndo *>`, `BSScrapArray<BGSHeadPart *, 1024>`,
/// `BSSimpleArray<BGSHeadPart *, 1024>`).
const VTABLE_NI_T_ARRAY: u32 = 0x0104_aae0;
const VTABLE_NI_T_PRIMITIVE_ARRAY: u32 = 0x0104_aae8;
const VTABLE_BS_SCRAP_ARRAY: u32 = 0x0104_aaf0;
const VTABLE_BS_SIMPLE_ARRAY: u32 = 0x0104_ab04;

/// Runs `body` with `words` stored one after the other in game memory (the
/// exe's wrappers take the address of their own arguments); `extra` more
/// zeroed bytes follow for locals. The closure receives the address of the
/// first word.
fn with_argument_words<R>(
    e: &mut Engine,
    words: &[u32],
    extra: u32,
    body: impl FnOnce(&mut Engine, u32) -> R,
) -> R {
    e.with_stack(words.len() as u32 * 4 + extra, |e, block| {
        let base = block.addr();
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *word);
        }
        for i in 0..extra {
            e.mem.set_u8(base + words.len() as u32 * 4 + i, 0);
        }
        body(e, base)
    })
}

/// `*mut (element pointers)` read through the exe's `0065fe40` wrapper.
fn unwrap_iterator(e: &mut Engine, pointer_to_word: u32) -> u32 {
    e.call(UNWRAP_ITERATOR, &args![pointer_to_word]).u32()
}

/// The overload tag byte the exe's `0065e750` returns.
fn overload_tag(e: &mut Engine, pointers: &[u32]) -> u32 {
    e.call(OVERLOAD_TAG, pointers).u32() & 0xff
}

layout! {
    /// `FR2MatrixVTC<float>` (Xbox PDB: `data` vector, `nrows`, `ncols`), `0x20`
    /// bytes on PC: the `std::vector<float>` is `0x18` bytes there (its
    /// allocator sits at `+8`, `_Myfirst` / `_Mylast` / `_Myend` at `+0xc`,
    /// `+0x10`, `+0x14`), then the row and column counts.
    pub struct Fr2Matrix: 0x20 {
        /// `std::vector::_Myfirst` of `data`.
        0x0C first: u32,
        /// `std::vector::_Mylast` of `data`.
        0x10 last: u32,
        /// `std::vector::_Myend` of `data`.
        0x14 end_of_storage: u32,
        /// `nrows` (Xbox PDB).
        0x18 nrows: u32,
        /// `ncols` (Xbox PDB).
        0x1C ncols: u32,
    }
}

// Translated from 0060b210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::SetFlagBit(0x1000, flag, 1)` on an actor-base
/// component (the `0x1000` bit, the counterpart of `fn_0060b1f0`'s test).
pub fn fn_0060b210(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(0x0047_dd50, &args![this, 0x1000u32, flag as u32, 1u32]);
}

// Translated from 0060b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the NPC a new race (the `TESRaceForm` component, `006ecd40`) unless it
/// already has it. The change flag `0x2000000` is cleared (virtual `0x4c`) when
/// the new race is the original race (which is then forgotten) and set
/// (virtual `0x48`) otherwise, the first race being remembered as the original
/// one; a height that followed the old race follows the new one; `actor` (when
/// not null) is refreshed through `008b78c0(actor, 0)`.
pub fn fn_0060b240(e: &mut Engine, this: Ptr<TESNPC>, race: Ptr, actor: Ptr) {
    let base = this.addr();
    let current = e.call(GET_WORD_AT_4, &args![base + COMPONENT_RACE]).u32();
    if race.addr() == current {
        return;
    }
    let race_height = tesnpc_get_race_height(e, this);
    let follows_race = f64::from(e.get(this, TESNPC::fHeight)) == f64::from(race_height);
    if e.get(this, TESNPC::pOriginalRace).addr() == race.addr() {
        e.call(SET_WORD_AT_4, &args![base + COMPONENT_RACE, race]);
        e.set(this, TESNPC::pOriginalRace, Ptr::NULL);
        e.vcall(base, 0x4c, &args![0x200_0000u32]);
    } else {
        e.call(SET_WORD_AT_4, &args![base + COMPONENT_RACE, race]);
        e.vcall(base, 0x48, &args![0x200_0000u32]);
        if e.get(this, TESNPC::pOriginalRace).is_null() {
            e.set(this, TESNPC::pOriginalRace, Ptr::new(current));
        }
    }
    if follows_race {
        let height = tesnpc_get_race_height(e, this);
        e.set(this, TESNPC::fHeight, height);
    }
    if !actor.is_null() {
        e.call(0x008b_78c0, &args![actor, 0u32]);
    }
}

// Translated from 0060b340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FR2MatrixVTC<float>::Resize`-like body (the map's name
/// `std::vector<float>::resize` is another instance folded onto it): stores
/// the row and column counts and resizes the vector to `rows * columns`
/// elements filled with 0.0 (`fn_0060b3f0`).
pub fn fn_0060b340(e: &mut Engine, this: Ptr<Fr2Matrix>, rows: u32, columns: u32) {
    e.set(this, Fr2Matrix::nrows, rows);
    e.set(this, Fr2Matrix::ncols, columns);
    fn_0060b3f0(e, this, rows.wrapping_mul(columns));
}

// Translated from 0060b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FR2MatrixVTC<float>::operator[]`: writes into `out` the row view
/// (`RowT`: pointer to the row's first element, then the column count) of row
/// `row`, through the row constructor `004b0680`, and returns `out`.
pub fn fn_0060b370(e: &mut Engine, this: Ptr<Fr2Matrix>, out: Ptr, row: u32) -> Ptr {
    let columns = e.get(this, Fr2Matrix::ncols);
    let data = e.call(VECTOR_ELEMENT_ADDRESS, &args![this, 0u32]).u32();
    let row_start = data.wrapping_add(
        row.wrapping_mul(e.get(this, Fr2Matrix::ncols))
            .wrapping_mul(4),
    );
    e.call(0x004b_0680, &args![out, row_start, columns]);
    out
}

// Translated from 0060b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes the matrix: `memset(&data[0], 0, rows * columns * 4)`.
pub fn fn_0060b3b0(e: &mut Engine, this: Ptr<Fr2Matrix>) {
    let size = e
        .get(this, Fr2Matrix::nrows)
        .wrapping_mul(e.get(this, Fr2Matrix::ncols))
        .wrapping_mul(4);
    let data = e.call(VECTOR_ELEMENT_ADDRESS, &args![this, 0u32]).u32();
    e.call(MEMSET, &args![data, 0u32, size]);
}

// Translated from 0060b3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `vector<float>::resize(count)`: `resize(count, 0.0f)`.
pub fn fn_0060b3f0(e: &mut Engine, this: Ptr<Fr2Matrix>, count: u32) {
    fn_0060b410(e, this, count, 0.0);
}

// Translated from 0060b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `vector<float>::resize(count, value)`: grows with `_Insert_n` at `end()`
/// (`fn_0060b4d0`) or erases from `begin() + count` to `end()`.
pub fn fn_0060b410(e: &mut Engine, this: Ptr<Fr2Matrix>, count: u32, value: f32) {
    // The value is taken by reference: it lives in game memory. The rest of
    // the frame holds the iterators (8 bytes each): end, begin, the sum, the
    // erase result.
    e.with_stack(0x28, |e, frame| {
        let f = frame.addr();
        e.mem.set_f32(f, value);
        let size = e.call(VECTOR_SIZE, &args![this]).u32();
        if size < count {
            let end = e.call(VECTOR_END, &args![this, f + 8]).u32();
            let proxy = e.mem.u32(end);
            let position = e.mem.u32(end + 4);
            let size = e.call(VECTOR_SIZE, &args![this]).u32();
            fn_0060b4d0(
                e,
                this,
                proxy,
                position,
                count.wrapping_sub(size),
                Ptr::new(f),
            );
        } else {
            let size = e.call(VECTOR_SIZE, &args![this]).u32();
            if count < size {
                let end = e.call(VECTOR_END, &args![this, f + 8]).u32();
                let end_proxy = e.mem.u32(end);
                let end_position = e.mem.u32(end + 4);
                let begin = e.call(VECTOR_BEGIN, &args![this, f + 0x10]).u32();
                let target = fn_0060b820(e, Ptr::new(begin), Ptr::new(f + 0x18), count);
                let proxy = e.mem.u32(target.addr());
                let position = e.mem.u32(target.addr() + 4);
                e.call(
                    VECTOR_ERASE,
                    &args![this, f + 0x20, proxy, position, end_proxy, end_position],
                );
            }
        }
    });
}

// Translated from 0060b4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `vector<float>::_Insert_n(where, count, &value)`: inserts `count` copies of
/// the float `value` points to at the element pointer `where_pointer`. Throws
/// (`0065f640`) when the size would pass `max_size`; reallocates to 1.5 times
/// the capacity (or to the exact size when that is larger) when the capacity is
/// too small, otherwise shifts the tail up (three cases: the tail is shorter
/// than `count`, or not). The exception-unwinding frame is not translated.
///
/// `_unused_1` is the iterator's proxy word, never read.
pub fn fn_0060b4d0(
    e: &mut Engine,
    this: Ptr<Fr2Matrix>,
    _unused_1: u32,
    where_pointer: u32,
    count: u32,
    value: Ptr,
) {
    let base = this.addr();
    let mut capacity = fn_0060b860(e, this);
    if count == 0 {
        return;
    }
    let size = e.call(VECTOR_SIZE, &args![this]).u32();
    let max_size = e.call(VECTOR_MAX_SIZE, &args![this]).u32();
    if max_size.wrapping_sub(size) < count {
        e.call(VECTOR_LENGTH_ERROR, &args![]);
        return;
    }
    let size = e.call(VECTOR_SIZE, &args![this]).u32();
    if capacity < size.wrapping_add(count) {
        // Reallocate.
        let half = capacity >> 1;
        let max_size = e.call(VECTOR_MAX_SIZE, &args![this]).u32();
        capacity = if max_size.wrapping_sub(half) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        let size = e.call(VECTOR_SIZE, &args![this]).u32();
        if capacity < size.wrapping_add(count) {
            let size = e.call(VECTOR_SIZE, &args![this]).u32();
            capacity = size.wrapping_add(count);
        }
        let buffer = e.call(VECTOR_ALLOCATE, &args![base + 8, capacity]).u32();
        let first = e.get(this, Fr2Matrix::first);
        let cursor = fn_0060bb40(e, this, first, where_pointer, buffer);
        let cursor = fn_0060b8a0(e, this, cursor, count, value);
        let last = e.get(this, Fr2Matrix::last);
        fn_0060bb40(e, this, where_pointer, last, cursor);
        let new_size = e.call(VECTOR_SIZE, &args![this]).u32().wrapping_add(count);
        let old_first = e.get(this, Fr2Matrix::first);
        if old_first != 0 {
            let old_last = e.get(this, Fr2Matrix::last);
            e.call(VECTOR_DESTROY, &args![this, old_first, old_last]);
            let first = e.get(this, Fr2Matrix::first);
            let old_capacity =
                ((e.get(this, Fr2Matrix::end_of_storage).wrapping_sub(first)) as i32 >> 2) as u32;
            e.call(
                VECTOR_DEALLOCATE,
                &args![base + 8, e.get(this, Fr2Matrix::first), old_capacity],
            );
        }
        e.set(
            this,
            Fr2Matrix::end_of_storage,
            buffer.wrapping_add(capacity.wrapping_mul(4)),
        );
        e.set(
            this,
            Fr2Matrix::last,
            buffer.wrapping_add(new_size.wrapping_mul(4)),
        );
        e.set(this, Fr2Matrix::first, buffer);
        return;
    }
    let last = e.get(this, Fr2Matrix::last);
    let tail = (last.wrapping_sub(where_pointer) as i32 >> 2) as u32;
    // The value is copied through the x87 before the elements move (it may
    // live inside the vector).
    let copy = x87_quiet_bits(e.mem.u32(value.addr()));
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, copy);
        if tail < count {
            let last = e.get(this, Fr2Matrix::last);
            fn_0060bb40(
                e,
                this,
                where_pointer,
                last,
                where_pointer.wrapping_add(count.wrapping_mul(4)),
            );
            let last = e.get(this, Fr2Matrix::last);
            let existing =
                (e.get(this, Fr2Matrix::last).wrapping_sub(where_pointer) as i32 >> 2) as u32;
            fn_0060b8a0(e, this, last, count.wrapping_sub(existing), Ptr::new(slot));
            let new_last = e
                .get(this, Fr2Matrix::last)
                .wrapping_add(count.wrapping_mul(4));
            e.set(this, Fr2Matrix::last, new_last);
            fn_0060bb70(
                e,
                where_pointer,
                new_last.wrapping_sub(count.wrapping_mul(4)),
                slot,
            );
        } else {
            let last = e.get(this, Fr2Matrix::last);
            let old_tail_start = last.wrapping_sub(count.wrapping_mul(4));
            let destination = e.get(this, Fr2Matrix::last);
            let new_last = fn_0060bb40(e, this, old_tail_start, last, destination);
            e.set(this, Fr2Matrix::last, new_last);
            fn_0060bba0(e, where_pointer, old_tail_start, last);
            fn_0060bb70(
                e,
                where_pointer,
                where_pointer.wrapping_add(count.wrapping_mul(4)),
                slot,
            );
        }
    });
}

// Translated from 0060b820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Iterator addition (`operator+`): copies the iterator at `this` (8 bytes:
/// proxy, element pointer), advances the copy by `count` elements
/// (`fn_0060b8e0`), stores it in `out` and returns `out`.
pub fn fn_0060b820(e: &mut Engine, this: Ptr, out: Ptr, count: u32) -> Ptr {
    let proxy = e.mem.u32(this.addr());
    let position = e.mem.u32(this.addr() + 4);
    e.with_stack(8, |e, copy| {
        e.mem.set_u32(copy.addr(), proxy);
        e.mem.set_u32(copy.addr() + 4, position);
        let moved = fn_0060b8e0(e, copy, count);
        let moved_proxy = e.mem.u32(moved.addr());
        let moved_position = e.mem.u32(moved.addr() + 4);
        e.mem.set_u32(out.addr(), moved_proxy);
        e.mem.set_u32(out.addr() + 4, moved_position);
    });
    out
}

// Translated from 0060b860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `vector<float>::capacity()`: 0 without a buffer, else
/// `(_Myend - _Myfirst) >> 2`.
pub fn fn_0060b860(e: &mut Engine, this: Ptr<Fr2Matrix>) -> u32 {
    if e.get(this, Fr2Matrix::first) == 0 {
        0
    } else {
        (e.get(this, Fr2Matrix::end_of_storage)
            .wrapping_sub(e.get(this, Fr2Matrix::first)) as i32
            >> 2) as u32
    }
}

// Translated from 0060b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `vector<float>::_Ufill(destination, count, &value)`: fills `count` floats at
/// `destination` with the allocator at `this + 8` (`fn_0060bc10`) and returns
/// the address after them.
pub fn fn_0060b8a0(
    e: &mut Engine,
    this: Ptr<Fr2Matrix>,
    destination: u32,
    count: u32,
    value: Ptr,
) -> u32 {
    fn_0060bc10(e, destination, count, value.addr(), this.addr() + 8);
    destination.wrapping_add(count.wrapping_mul(4))
}

// Translated from 0060b8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Iterator `operator+=`: advances by `count` (`fn_0060b900`) and returns
/// the iterator.
pub fn fn_0060b8e0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    fn_0060b900(e, this, count);
    this
}

// Translated from 0060b900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checked iterator advance: `_invalid_parameter` (`00ec7c56`) when the
/// iterator has no container (`005ca4f0`) or when the new element pointer
/// leaves `[first, last]` of its container (`fn_0060b980`), then adds `count *
/// 4` to the element pointer.
pub fn fn_0060b900(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    let base = this.addr();
    if !e.call(0x005c_a4f0, &args![this]).bool() {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let target = e.mem.u32(base + 4).wrapping_add(count.wrapping_mul(4));
    let container = fn_0060b980(e, this);
    let mut invalid = target > e.mem.u32(container + 0x10);
    if !invalid {
        let target = e.mem.u32(base + 4).wrapping_add(count.wrapping_mul(4));
        let container = fn_0060b980(e, this);
        invalid = target < e.mem.u32(container + 0xc);
    }
    if invalid {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let moved = e.mem.u32(base + 4).wrapping_add(count.wrapping_mul(4));
    e.mem.set_u32(base + 4, moved);
    this
}

// Translated from 0060b980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container of a checked iterator: 0 without a proxy, else the word the
/// proxy points to (`00559450`).
pub fn fn_0060b980(e: &mut Engine, this: Ptr) -> u32 {
    let proxy = e.mem.u32(this.addr());
    if proxy == 0 {
        0
    } else {
        e.call(GET_FIRST_WORD, &args![proxy]).u32()
    }
}

// Translated from 0060b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `NiTArray<FaceGenUndo *, ...>` base (the same
/// code serves the `basic_streambuf` destructors of the CRT, hence the
/// library name in the map): restores the base vtable and frees the buffer
/// (`004ede70`).
pub fn fn_0060b9b0(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), VTABLE_NI_T_ARRAY);
    let buffer = e.get(this, NiTArray::m_pBase);
    e.call(0x004e_de70, &args![buffer]);
}

// Translated from 0060b9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<FaceGenUndo *>` constructor (the NPC's
/// `FaceGenUndoStates`): the `NiTArray` constructor (`fn_0060bd20`), then the
/// derived vtable. The two words are the maximum size and the growth step.
pub fn fn_0060b9e0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTArray> {
    fn_0060bd20(e, this, max_size as u16, grow_by as u16);
    e.mem.set_u32(this.addr(), VTABLE_NI_T_PRIMITIVE_ARRAY);
    this
}

// Translated from 0060ba10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray::Add(value)`: stores `value` at index `m_usSize` (`fn_0060bd90`)
/// and returns that index.
pub fn fn_0060ba10(e: &mut Engine, this: Ptr<NiTArray>, value: u32) -> u32 {
    let index = u32::from(e.get(this, NiTArray::m_usSize));
    fn_0060bd90(e, this, index, value)
}

// Translated from 0060ba40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<BGSHeadPart *, 1024>` constructor: the `BSSimpleArray` base
/// (`fn_0060bdd0`), the scrap-array vtable, the thread's scrap heap
/// (`MemoryManager::GetThreadScrapHeap`) at `+0x10`, and the base's
/// `006b3eb0(0, 0)` again. The exception-unwinding frame is not translated.
pub fn fn_0060ba40(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0060bdd0(e, this);
    e.mem.set_u32(this.addr(), VTABLE_BS_SCRAP_ARRAY);
    let manager = e.call(MEMORY_MANAGER, &args![]).u32();
    let heap = e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32();
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0060bac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSHeadPart *, 1024>` destructor: restores its vtable and
/// releases the buffer (`008454f0(this, 1)`).
pub fn fn_0060bac0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_BS_SIMPLE_ARRAY);
    e.call(0x0084_54f0, &args![this, 1u32]);
}

// Translated from 0060bae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<BGSHeadPart *, 1024>` destructor: its own vtable, the buffer
/// release (`008454f0(this, 1)`), then the `BSSimpleArray` destructor
/// (`fn_0060bac0`). The exception-unwinding frame is not translated.
pub fn fn_0060bae0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_BS_SCRAP_ARRAY);
    e.call(0x0084_54f0, &args![this, 1u32]);
    fn_0060bac0(e, this);
}

// Translated from 0060bb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `vector<float>::_Ucopy(first, last, destination)`: `fn_0060be00` with
/// the allocator at `this + 8`; returns the end of the copy.
pub fn fn_0060bb40(
    e: &mut Engine,
    this: Ptr<Fr2Matrix>,
    first: u32,
    last: u32,
    destination: u32,
) -> u32 {
    fn_0060be00(e, first, last, destination, this.addr() + 8)
}

// Translated from 0060bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fill(first, last, &value)` with unchecked iterators: unwraps both ends
/// (`0065fe40`) and runs `fn_0060be50`.
pub fn fn_0060bb70(e: &mut Engine, first: u32, last: u32, value: u32) {
    with_argument_words(e, &[first, last], 0, |e, words| {
        let last_unwrapped = unwrap_iterator(e, words + 4);
        let first_unwrapped = unwrap_iterator(e, words);
        fn_0060be50(e, first_unwrapped, last_unwrapped, value);
    });
}

// Translated from 0060bba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copy_backward(first, last, destinationEnd)` with unchecked iterators:
/// picks the overload tag (`0065e750`, `0065fd70`), unwraps both ends and runs
/// `fn_0060be80`.
pub fn fn_0060bba0(e: &mut Engine, first: u32, last: u32, destination_end: u32) {
    with_argument_words(e, &[first, last, destination_end], 4, |e, words| {
        let tag = overload_tag(e, &[words + 8]);
        let local = e
            .call(
                RETURN_FIRST_ARGUMENT,
                &args![words + 12, words, words + 8, tag, 0u32],
            )
            .u32();
        let local_tag = u32::from(e.mem.u8(local));
        let last_unwrapped = unwrap_iterator(e, words + 4);
        let first_unwrapped = unwrap_iterator(e, words);
        fn_0060be80(
            e,
            first_unwrapped,
            last_unwrapped,
            destination_end,
            local_tag,
            tag,
            0,
        );
    });
}

// Translated from 0060bc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_Uninitialized_fill_n(first, count, &value, allocator)`: takes the overload
/// tag (`0065e750`) and runs `fn_0060bed0`.
pub fn fn_0060bc10(e: &mut Engine, first: u32, count: u32, value: u32, allocator: u32) {
    with_argument_words(e, &[first], 0, |e, words| {
        let tag = overload_tag(e, &[words, words]);
        fn_0060bed0(e, first, count, value, allocator, tag, 0);
    });
}

// Translated from 0060bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTArray<FaceGenUndo *>` vtable: runs
/// `fn_0060b9b0` and, with bit 0 of `flags`, frees the object (`00401030`).
pub fn fn_0060bc60(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    fn_0060b9b0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 0060bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTPrimitiveArray<FaceGenUndo *>` vtable:
/// the destructor body `006013a0`, then the free with bit 0 of `flags`.
pub fn fn_0060bc90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0060_13a0, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 0060bcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSHeadPart *, 1024>::scalar deleting destructor` (Xbox
/// PDB): `fn_0060bac0`, then the free with bit 0 of `flags`.
pub fn bs_simple_array_bgs_head_part_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0060bac0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 0060bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<BGSHeadPart *, 1024>::scalar deleting destructor` (Xbox PDB):
/// `fn_0060bae0`, then the free with bit 0 of `flags`.
pub fn bs_scrap_array_bgs_head_part_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0060bae0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 0060bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<FaceGenUndo *, ...>` constructor: base vtable, maximum size,
/// growth step, no elements, and a buffer of `max_size` words (`0096afc0`)
/// unless the maximum size is 0.
pub fn fn_0060bd20(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), VTABLE_NI_T_ARRAY);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    if max_size != 0 {
        let buffer = e.call(0x0096_afc0, &args![u32::from(max_size)]).u32();
        e.set(this, NiTArray::m_pBase, buffer);
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    this
}

// Translated from 0060bd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray::SetAt(index, value)` with growth: when `index` is not below
/// the maximum size, the array grows to `index + growBy` first (`0060bef0`);
/// then the element is stored (`0060c120`). Returns `index`.
pub fn fn_0060bd90(e: &mut Engine, this: Ptr<NiTArray>, index: u32, value: u32) -> u32 {
    if index >= u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        let new_size = u32::from(e.get(this, NiTArray::m_usGrowBy)).wrapping_add(index);
        e.call(ARRAY_GROW, &args![this, new_size]);
    }
    e.call(ARRAY_STORE, &args![this, index, value]);
    index
}

// Translated from 0060bdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSHeadPart *, 1024>` constructor: its vtable and
/// `006b3eb0(0, 0)`.
pub fn fn_0060bdd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_BS_SIMPLE_ARRAY);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0060be00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_Uninitialized_copy(first, last, destination, allocator)` with unchecked
/// iterators: takes the overload tag (`0065e750`), unwraps both ends and runs
/// `0060c270`; returns its result (the end of the copy).
pub fn fn_0060be00(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    with_argument_words(e, &[first, last, destination], 0, |e, words| {
        let tag = overload_tag(e, &[words + 8]);
        let last_unwrapped = unwrap_iterator(e, words + 4);
        let first_unwrapped = unwrap_iterator(e, words);
        e.call(
            UNINITIALIZED_COPY,
            &args![
                first_unwrapped,
                last_unwrapped,
                destination,
                allocator,
                tag,
                0u32
            ],
        )
        .u32()
    })
}

// Translated from 0060be50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `for (p = first; p != last; ++p) *p = *value;` on floats, each copy
/// going through the x87 (`FLD` / `FSTP`).
pub fn fn_0060be50(e: &mut Engine, first: u32, last: u32, value: u32) {
    let mut pointer = first;
    while pointer != last {
        let bits = x87_quiet_bits(e.mem.u32(value));
        e.mem.set_u32(pointer, bits);
        pointer = pointer.wrapping_add(4);
    }
}

// Translated from 0060be80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_Copy_backward(first, last, destinationEnd, ...)` with unchecked iterators:
/// takes the overload tag (`0065e750(&first, &destinationEnd)`) and runs
/// `0060c290`. `tag_byte` is read as a byte; the last two words are never read.
pub fn fn_0060be80(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination_end: u32,
    tag_byte: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    with_argument_words(e, &[first, last, destination_end], 0, |e, words| {
        let tag = overload_tag(e, &[words, words + 8]);
        e.call(
            COPY_BACKWARD,
            &args![first, last, destination_end, tag_byte & 0xff, tag, 0u32],
        );
    });
}

// Translated from 0060bed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_Uninitialized_fill_n` dispatch: forwards the first three words to
/// `0060c2e0`. The last three words (allocator and two tags) are never read.
pub fn fn_0060bed0(
    e: &mut Engine,
    first: u32,
    count: u32,
    value: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    e.call(UNINITIALIZED_FILL, &args![first, count, value]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00601170, tesnpc_tesnpc(Ptr<TESNPC>) -> Ptr<TESNPC>),
        entry!(0x00601570, tesnpc_initialize_data(Ptr<TESNPC>)),
        entry!(0x00602190, tesnpc_load(Ptr<TESNPC>, Ptr) -> bool),
        entry!(0x006031e0, fn_006031e0(Ptr<TESNPC>, Ptr)),
        entry!(0x00603200, fn_00603200(Ptr<TESNPC>, Ptr)),
        entry!(0x00603220, fn_00603220(Ptr, Ptr) -> i32),
        entry!(0x00603280, fn_00603280(Ptr, i32, Ptr)),
        entry!(0x00603370, tesnpc_init_item(Ptr<TESNPC>)),
        entry!(0x006035e0, tesnpc_copy(Ptr<TESNPC>, Ptr<TESNPC>)),
        entry!(0x00603790, fn_00603790(Ptr<TESNPC>, Ptr<TESNPC>)),
        entry!(0x00603880, tesnpc_compare(Ptr<TESNPC>, Ptr) -> bool),
        entry!(0x00603ad0, tesnpc_get_face_coord(Ptr<TESNPC>, Ptr)),
        entry!(0x00603b50, fn_00603b50(Ptr<TESNPC>) -> Ptr),
        entry!(0x00603bb0, fn_00603bb0(Ptr<TESNPC>)),
        entry!(0x00603be0, tesnpc_init_values(Ptr<TESNPC>, bool)),
        entry!(0x00603f90, tesnpc_get_form_health(Ptr) -> i32),
        entry!(0x00603fc0, fn_00603fc0(Ptr<TESNPC>, bool) -> i32),
        entry!(0x006040d0, fn_006040d0(Ptr<TESNPC>, bool) -> i32),
        entry!(0x006041c0, fn_006041c0() -> u32),
        entry!(0x006041d0, tesnpc_get_race_height(Ptr<TESNPC>) -> f32),
        entry!(0x00604210, fn_00604210(Ptr, i32) -> f32),
        entry!(0x00604250, tesnpc_get_weight(Ptr<TESNPC>) -> f32),
        entry!(0x006042d0, fn_006042d0(Ptr, i32) -> f32),
        entry!(
            0x00604310,
            tesnpc_get_head_part_mod_texture_file_name(Ptr<TESNPC>, i32, Ptr, u32) -> bool
        ),
        entry!(0x006044c0, fn_006044c0(Ptr<TESNPC>) -> Ptr<TESNPC>),
        entry!(0x006044e0, fn_006044e0(Ptr<TESNPC>, i32, Ptr)),
        entry!(
            0x006045f0,
            bs_face_gen_manager_get_default_base_modulation_texture() -> Ptr
        ),
        entry!(0x006046a0, tesnpc_get_voice_type(Ptr) -> Ptr),
        entry!(0x00604780, fn_00604780(Ptr, i32) -> Ptr),
        entry!(0x006047a0, fn_006047a0(Ptr) -> u8),
        entry!(
            0x006047c0,
            fn_006047c0(Ptr<TESNPC>, Ptr, u32, bool, u32, bool)
        ),
        entry!(0x00604ba0, fn_00604ba0(Ptr, Ptr)),
        entry!(0x006053f0, fn_006053f0(Ptr<TESNPC>, f32)),
        entry!(0x00605410, tesnpc_clone_3d(Ptr<TESNPC>, Ptr) -> Ptr),
        entry!(0x006055d0, fn_006055d0(Ptr<TESNPC>, u32) -> u32),
        entry!(
            0x006055f0,
            tesnpc_create_biped_anim(Ptr<TESNPC>, Ptr) -> Ptr
        ),
        entry!(0x006056f0, fn_006056f0(Ptr<TESNPC>, Ptr)),
        entry!(0x00605d20, fn_00605d20() -> u8),
        entry!(0x00605d40, fn_00605d40(u32) -> u32),
        entry!(0x00605d50, fn_00605d50() -> u8),
        entry!(0x00605d70, tesnpc_replace_ref_model(Ptr<TESNPC>, Ptr)),
        entry!(0x00605e70, fn_00605e70(Ptr<TESNPC>, Ptr, Ptr, Ptr)),
        entry!(
            0x00605fc0,
            tesnpc_build_object_array(Ptr<TESNPC>, u32, Ptr, Ptr)
        ),
        entry!(0x00606050, tesnpc_init_worn(Ptr<TESNPC>, Ptr, Ptr)),
        entry!(
            0x006061b0,
            tesnpc_init_worn_object(Ptr<TESNPC>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x006062e0, fn_006062e0(Ptr<TESNPC>, Ptr, Ptr)),
        entry!(0x00606540, fn_00606540(Ptr<TESNPC>, Ptr, Ptr, u8)),
        entry!(0x00606800, fn_00606800(Ptr) -> u32),
        entry!(
            0x00606820,
            tesnpc_linear_face_gen_head_load(Ptr<TESNPC>, Ptr, Ptr)
        ),
        entry!(0x006072c0, fn_006072c0(Ptr) -> u32),
        entry!(0x00607310, fn_00607310(u32) -> u32),
        entry!(0x00607340, fn_00607340() -> u32),
        entry!(0x00607350, fn_00607350(Ptr) -> u16),
        entry!(0x00607370, tesnpc_init_head(Ptr<TESNPC>, Ptr, Ptr)),
        entry!(0x00607420, fn_00607420(Ptr<TESNPC>, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00607810, fn_00607810(Ptr, u32, f32)),
        entry!(0x00607830, fn_00607830(Ptr, f32)),
        entry!(0x00607850, fn_00607850(Ptr, u32) -> u32),
        entry!(0x006078e0, fn_006078e0(Ptr<TESNPC>, u32, u32)),
        entry!(0x00607950, fn_00607950(Ptr<TESNPC>) -> Ptr),
        entry!(0x00607970, fn_00607970(Ptr<TESNPC>, Ptr)),
        entry!(
            0x00607990,
            tesnpc_activate(Ptr<TESNPC>, Ptr, Ptr, u32, Ptr, u32) -> bool
        ),
        entry!(0x00608d80, fn_00608d80(Ptr) -> bool),
        entry!(0x00608da0, fn_00608da0(Ptr<TESNPC>, u32) -> u16),
        entry!(0x00608e00, fn_00608e00(Ptr<TESNPC>, u32)),
        entry!(0x00608e80, fn_00608e80(Ptr<TESNPC>, u32, u32)),
        entry!(0x00608f00, fn_00608f00(Ptr<TESNPC>, Ptr)),
        entry!(0x00609220, fn_00609220(Ptr<TESNPC>, Ptr)),
        entry!(0x006099f0, fn_006099f0(Ptr<TESNPC>, Ptr)),
        entry!(0x00609bf0, fn_00609bf0(Ptr<TESNPC>, Ptr) -> bool),
        entry!(0x00609c70, fn_00609c70(Ptr<TESNPC>, u32) -> u16),
        entry!(0x00609d60, fn_00609d60(Ptr<TESNPC>, u32)),
        entry!(0x00609f60, fn_00609f60(Ptr<TESNPC>, Ptr)),
        entry!(0x0060a890, fn_0060a890(Ptr<TESNPC>, Ptr)),
        entry!(0x0060a950, fn_0060a950(Ptr<TESNPC>, Ptr)),
        entry!(0x0060aeb0, fn_0060aeb0(Ptr, u32)),
        entry!(0x0060af60, fn_0060af60(Ptr, u16)),
        entry!(
            0x0060af90,
            tesnpc_build_default_model_list(Ptr<TESNPC>, u8, u8) -> Ptr
        ),
        entry!(0x0060b1d0, fn_0060b1d0(Ptr) -> u32),
        entry!(0x0060b1f0, fn_0060b1f0(Ptr) -> u32),
        entry!(0x0060b210, fn_0060b210(Ptr, u8)),
        entry!(0x0060b240, fn_0060b240(Ptr<TESNPC>, Ptr, Ptr)),
        entry!(0x0060b340, fn_0060b340(Ptr<Fr2Matrix>, u32, u32)),
        entry!(0x0060b370, fn_0060b370(Ptr<Fr2Matrix>, Ptr, u32) -> Ptr),
        entry!(0x0060b3b0, fn_0060b3b0(Ptr<Fr2Matrix>)),
        entry!(0x0060b3f0, fn_0060b3f0(Ptr<Fr2Matrix>, u32)),
        entry!(0x0060b410, fn_0060b410(Ptr<Fr2Matrix>, u32, f32)),
        entry!(0x0060b4d0, fn_0060b4d0(Ptr<Fr2Matrix>, u32, u32, u32, Ptr)),
        entry!(0x0060b820, fn_0060b820(Ptr, Ptr, u32) -> Ptr),
        entry!(0x0060b860, fn_0060b860(Ptr<Fr2Matrix>) -> u32),
        entry!(
            0x0060b8a0,
            fn_0060b8a0(Ptr<Fr2Matrix>, u32, u32, Ptr) -> u32
        ),
        entry!(0x0060b8e0, fn_0060b8e0(Ptr, u32) -> Ptr),
        entry!(0x0060b900, fn_0060b900(Ptr, u32) -> Ptr),
        entry!(0x0060b980, fn_0060b980(Ptr) -> u32),
        entry!(0x0060b9b0, fn_0060b9b0(Ptr<NiTArray>)),
        entry!(
            0x0060b9e0,
            fn_0060b9e0(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
        ),
        entry!(0x0060ba10, fn_0060ba10(Ptr<NiTArray>, u32) -> u32),
        entry!(0x0060ba40, fn_0060ba40(Ptr) -> Ptr),
        entry!(0x0060bac0, fn_0060bac0(Ptr)),
        entry!(0x0060bae0, fn_0060bae0(Ptr)),
        entry!(
            0x0060bb40,
            fn_0060bb40(Ptr<Fr2Matrix>, u32, u32, u32) -> u32
        ),
        entry!(0x0060bb70, fn_0060bb70(u32, u32, u32)),
        entry!(0x0060bba0, fn_0060bba0(u32, u32, u32)),
        entry!(0x0060bc10, fn_0060bc10(u32, u32, u32, u32)),
        entry!(0x0060bc60, fn_0060bc60(Ptr<NiTArray>, u32) -> Ptr<NiTArray>),
        entry!(0x0060bc90, fn_0060bc90(Ptr, u32) -> Ptr),
        entry!(
            0x0060bcc0,
            bs_simple_array_bgs_head_part_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0060bcf0,
            bs_scrap_array_bgs_head_part_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0060bd20,
            fn_0060bd20(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(0x0060bd90, fn_0060bd90(Ptr<NiTArray>, u32, u32) -> u32),
        entry!(0x0060bdd0, fn_0060bdd0(Ptr) -> Ptr),
        entry!(0x0060be00, fn_0060be00(u32, u32, u32, u32) -> u32),
        entry!(0x0060be50, fn_0060be50(u32, u32, u32)),
        entry!(0x0060be80, fn_0060be80(u32, u32, u32, u32, u32, u32)),
        entry!(0x0060bed0, fn_0060bed0(u32, u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const EXTERNAL_CALLEES: [u32; 235] = [
        0x0040_1000,
        0x0040_1080,
        0x0040_1170,
        0x0040_13e0,
        0x0040_1460,
        0x0040_1660,
        0x0040_1680,
        0x0040_3570,
        0x0040_3d30,
        0x0040_3e20,
        0x0040_4040,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0040_6d00,
        0x0040_6d70,
        0x0040_6d90,
        0x0040_7a90,
        0x0040_8b20,
        0x0040_8d60,
        0x0040_8da0,
        0x0040_e690,
        0x0040_e780,
        0x0041_3f40,
        0x0041_69d0,
        0x0043_b1b0,
        0x0043_b4a0,
        0x0043_d410,
        0x0043_d4d0,
        0x0044_1110,
        0x0044_59e0,
        0x0044_ddc0,
        0x0045_3470,
        0x0045_64f0,
        0x0045_68c0,
        0x0045_bc00,
        0x0045_cec0,
        0x0046_1560,
        0x0046_1580,
        0x0046_2230,
        0x0046_23f0,
        0x0046_7bb0,
        0x0047_0470,
        0x0047_2660,
        0x0047_26b0,
        0x0047_26f0,
        0x0047_27f0,
        0x0047_2840,
        0x0047_2890,
        0x0047_81e0,
        0x0047_86e0,
        0x0047_8be0,
        0x0047_bdd0,
        0x0047_beb0,
        0x0047_c850,
        0x0047_ca20,
        0x0047_cb90,
        0x0047_cca0,
        0x0047_ccc0,
        0x0047_cd40,
        0x0047_cd90,
        0x0047_cdb0,
        0x0047_cdd0,
        0x0047_ce10,
        0x0047_d1a0,
        0x0047_d220,
        0x0047_d370,
        0x0047_d390,
        0x0047_d3b0,
        0x0047_d3d0,
        0x0047_d800,
        0x0047_d8b0,
        0x0047_dd30,
        0x0047_dd50,
        0x0047_de40,
        0x0047_de70,
        0x0047_dea0,
        0x0047_ded0,
        0x0047_dfe0,
        0x0047_e010,
        0x0047_e1c0,
        0x0047_e270,
        0x0047_ee00,
        0x0047_eed0,
        0x0047_ef00,
        0x0047_ef50,
        0x0047_efb0,
        0x0047_f060,
        0x0047_f0b0,
        0x0047_f190,
        0x0047_f1e0,
        0x0047_f2d0,
        0x0047_f4c0,
        0x0047_f500,
        0x0047_faa0,
        0x0047_fb50,
        0x0048_0000,
        0x0048_0160,
        0x0048_0180,
        0x0048_0af0,
        0x0048_0db0,
        0x0048_17f0,
        0x0048_1bb0,
        0x0048_1c80,
        0x0048_3170,
        0x0048_39c0,
        0x0048_4ab0,
        0x0048_4e60,
        0x0048_4ee0,
        0x0048_5110,
        0x0048_5270,
        0x0048_5d50,
        0x0048_61f0,
        0x0048_6620,
        0x0048_7050,
        0x0048_70f0,
        0x0048_7240,
        0x0048_92d0,
        0x0048_9430,
        0x0048_b800,
        0x0048_b870,
        0x0048_b920,
        0x0048_cce0,
        0x0048_cd90,
        0x0048_d290,
        0x0048_d400,
        0x0048_d630,
        0x004a_aca0,
        0x004a_ad00,
        0x004a_ae30,
        0x004a_c110,
        0x004b_3e60,
        0x004b_f220,
        0x004b_fe50,
        0x004c_0cd0,
        0x004c_7400,
        0x004c_8220,
        0x004e_a950,
        0x004f_15a0,
        0x0050_2430,
        0x0050_3210,
        0x0050_3240,
        0x0050_3650,
        0x0050_5000,
        0x0050_eaa0,
        0x0050_ee50,
        0x0050_f9a0,
        0x0050_f9c0,
        0x0050_fbf0,
        0x0055_9450,
        0x0056_5210,
        0x0056_afc0,
        0x0057_1b50,
        0x0057_5400,
        0x0057_5690,
        0x0058_db10,
        0x0059_bb30,
        0x005a_5f40,
        0x005a_e380,
        0x005a_e3d0,
        0x005b_5e40,
        0x005d_1070,
        0x005d_9fb0,
        0x005d_d590,
        0x005d_d6f0,
        0x005f_0cc0,
        0x005f_1230,
        0x005f_65d0,
        0x005f_75e0,
        0x005f_7b50,
        0x0060_13c0,
        0x0060_1800,
        0x0060_1c10,
        0x0060_1c70,
        0x0060_2130,
        0x0060_2150,
        0x0060_2170,
        0x0060_b340,
        0x0060_b370,
        0x0060_b3b0,
        0x0060_b9e0,
        0x0060_ba10,
        0x0061_30e0,
        0x0061_3870,
        0x0061_3970,
        0x0061_3b20,
        0x0062_86d0,
        0x0062_9ab0,
        0x0063_3c90,
        0x0063_4890,
        0x0064_3bf0,
        0x0064_3c20,
        0x0064_8c10,
        0x0065_21e0,
        0x0065_21f0,
        0x0065_2900,
        0x0065_29a0,
        0x0065_2af0,
        0x0065_3270,
        0x0065_5fa0,
        0x0066_b0d0,
        0x0066_ec10,
        0x0068_15c0,
        0x006e_5cc0,
        0x006e_cd40,
        0x0070_37c0,
        0x0071_7e50,
        0x0072_6070,
        0x0072_9970,
        0x0076_b610,
        0x007a_f430,
        0x007c_b2e0,
        0x0082_56d0,
        0x0084_e3a0,
        0x0088_c830,
        0x0089_1170,
        0x008a_1710,
        0x008c_26e0,
        0x008d_8520,
        0x008f_21d0,
        0x0090_5820,
        0x0094_42e0,
        0x0094_4300,
        0x0096_11e0,
        0x0096_a2d0,
        0x0096_a610,
        0x00a5_9c60,
        0x00a5_9d30,
        0x00a5_a040,
        0x00ec_408c,
        0x00ec_43fb,
        0x00ec_4835,
        0x00ec_5ec0,
        0x00ec_62c0,
        0x00ec_782f,
        0x00ec_7ec0,
    ];

    // BEGIN second-callees
    /// Callees of the second block of functions (`ReplaceRefModel` to
    /// `0060b1f0`) that `EXTERNAL_CALLEES` does not list; `world()` makes them
    /// doubles returning 0.
    const SECOND_CALLEES: [u32; 165] = [
        0x0040_37b0,
        0x0040_37d0,
        0x0040_3df0,
        0x0040_6f60,
        0x0041_c930,
        0x0041_ca90,
        0x0041_cb10,
        0x0041_cb70,
        0x0041_d8a0,
        0x0042_80f0,
        0x0042_8110,
        0x0042_ce30,
        0x0042_ce90,
        0x0042_e8c0,
        0x0043_7bd0,
        0x0043_7bf0,
        0x0043_8170,
        0x0043_81b0,
        0x0043_9410,
        0x0043_b230,
        0x0043_b480,
        0x0043_f220,
        0x0043_fa80,
        0x0043_fad0,
        0x0043_faf0,
        0x0043_fcd0,
        0x0044_0460,
        0x0044_1420,
        0x0044_1b00,
        0x0045_0f90,
        0x0045_3a70,
        0x0047_0c70,
        0x0047_23a0,
        0x0047_5020,
        0x0048_0bd0,
        0x0048_11e0,
        0x0048_2720,
        0x0048_2910,
        0x0048_29c0,
        0x0048_4ce0,
        0x0048_4d00,
        0x0048_4d20,
        0x0048_4d40,
        0x0049_3bb0,
        0x0049_6940,
        0x0049_97b0,
        0x004a_b230,
        0x004a_b250,
        0x004a_b400,
        0x004a_bfa0,
        0x004a_c1e0,
        0x004a_d050,
        0x004a_d270,
        0x004a_dda0,
        0x004a_ddc0,
        0x004c_8c10,
        0x004e_af60,
        0x004f_3200,
        0x004f_32e0,
        0x004f_8960,
        0x0050_0940,
        0x0050_d100,
        0x0052_aa80,
        0x0053_7bd0,
        0x0054_95f0,
        0x0055_85e0,
        0x0055_d520,
        0x0056_6950,
        0x0056_8ad0,
        0x0056_fac0,
        0x0057_b7c0,
        0x005b_8fc0,
        0x005c_90d0,
        0x005d_43c0,
        0x005d_9f90,
        0x005d_9ff0,
        0x005d_d560,
        0x005f_0fb0,
        0x005f_12d0,
        0x005f_1590,
        0x005f_16f0,
        0x005f_18c0,
        0x005f_1b30,
        0x005f_1f30,
        0x005f_1fd0,
        0x005f_20a0,
        0x0060_ba40,
        0x0060_bae0,
        0x0061_3c50,
        0x0061_41f0,
        0x0061_9df0,
        0x0061_a2d0,
        0x0061_b320,
        0x0064_9f00,
        0x0064_9f70,
        0x0064_9fe0,
        0x0064_a070,
        0x0064_c5a0,
        0x0065_1b30,
        0x0065_7820,
        0x0066_ec80,
        0x0067_0f90,
        0x0067_27b0,
        0x0067_2800,
        0x0067_8610,
        0x0069_3d50,
        0x006a_7ad0,
        0x0070_2640,
        0x0070_52f0,
        0x0070_9470,
        0x0071_9b20,
        0x0075_4d90,
        0x0080_41a0,
        0x0081_5b00,
        0x0082_5c00,
        0x0083_c7b0,
        0x0083_c7e0,
        0x0086_48a0,
        0x0086_4980,
        0x0086_4a60,
        0x0086_5df0,
        0x0086_5e50,
        0x0086_5f20,
        0x0086_5ff0,
        0x0086_7d60,
        0x0086_7da0,
        0x0087_7a30,
        0x0087_f3d0,
        0x0089_4d60,
        0x008a_61b0,
        0x008a_6650,
        0x008a_7a90,
        0x008a_8e50,
        0x008a_ce90,
        0x008b_06d0,
        0x008b_3bb0,
        0x008b_3c30,
        0x008b_70d0,
        0x008b_b520,
        0x008c_00e0,
        0x008d_8ac0,
        0x0093_36c0,
        0x0093_44a0,
        0x0095_0b00,
        0x0095_0b30,
        0x0095_0bb0,
        0x0096_ae90,
        0x0097_fa10,
        0x0099_62f0,
        0x009a_4320,
        0x009c_8950,
        0x009d_ace0,
        0x00a5_b230,
        0x00a5_bdd0,
        0x00a5_d2c0,
        0x00a5_d510,
        0x00a6_9dd0,
        0x00a6_aa40,
        0x00a6_e870,
        0x00a6_e8e0,
        0x00aa_13e0,
        0x00ad_8780,
        0x00b5_7e30,
        0x00c7_48d0,
        0x00ec_623a,
    ];
    // END second-callees

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn ret_float(value: f32) -> Ret {
        value.into_ret()
    }

    /// An engine where every callee outside this file is a double returning
    /// zero (and recording its call), except the one-line accessors the
    /// compiler folded (`this + 4`, `this + 0x20`, ...), which get their
    /// real bodies, plus the CRT functions the code relies on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in EXTERNAL_CALLEES {
            if addr != 0x0040_1000 {
                e.register(addr, |_, _| Ret::default());
            }
        }
        // Folded accessors (their disassembly is in the notes of each use).
        e.register(GET_WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(GET_WORD_AT_20, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(GET_FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(TEST_ACTOR_FLAGS, |e, a| {
            ret((e.mem.u32(a[0] + 4) & a[1] != 0) as u32)
        });
        e.register(GET_SEX, |e, a| {
            if e.mem.u8(a[0] + 4) == 0x2a {
                ret((e.mem.u32(a[0] + 0x34) & 1 != 0) as u32)
            } else {
                ret(u32::MAX)
            }
        });
        e.register(NPC_GET_RACE, |e, a| ret(e.mem.u32(a[0] + 0x110)));
        e.register(GET_FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(NPC_GET_CLASS, |e, a| ret(e.mem.u32(a[0] + 0x130)));
        e.register(NPC_GET_DATA, |_, a| ret(a[0] + 0x114));
        e.register(NPC_GET_HEAD_PARTS, |_, a| ret(a[0] + 0x1dc));
        e.register(NPC_GET_HAIR_LENGTH, |e, a| {
            ret_float(e.mem.f32(a[0] + 0x1bc))
        });
        e.register(NPC_GET_EYE_COLOR, |e, a| ret(e.mem.u32(a[0] + 0x1c0)));
        e.register(NPC_GET_HAIR_COLOR, |e, a| ret(e.mem.u32(a[0] + 0x1d8)));
        e.register(NPC_FACE_COORD_POINTER, |e, a| {
            let alternate = e.mem.u32(a[0] + 0x1b4);
            ret(if alternate != 0 {
                alternate
            } else {
                a[0] + 0x134
            })
        });
        e.register(GET_FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(LIST_NODE_SELF, |_, a| ret(a[0]));
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0) as u32)
        });
        e.register(0x0094_4300, |e, a| ret_float(e.mem.f32(a[0] + 0x1f4)));
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            ret(a[0])
        });
        e.register(STRCMP, |e, a| {
            let first = e.mem.cstr(a[0]);
            let second = e.mem.cstr(a[1]);
            ret(match first.cmp(&second) {
                std::cmp::Ordering::Less => u32::MAX,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(FTOL, |_, a| {
            let value = f64::take(a, &mut 0);
            ret(value as i32 as u32)
        });
        e.register(GET_TEMPLATE_FORM, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        e.register(PROCESS_OF_ACTOR, |e, a| ret(e.mem.u32(a[0] + 0x68)));
        e.register(0x0044_ddc0, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(FILE_NAME, |_, a| ret(a[0] + 0x20));
        e.register(SET_WORD_AT_4, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(GET_LEVEL, |e, a| ret(e.mem.u16(a[0] + 8) as u32));
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(NI_POINTER_ASSIGN_POINTER, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(GET_INT_SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.register(GET_FLOAT_SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.register(GET_BOOL_SETTING_VALUE, |_, a| ret(a[0] + 4));
        // Pages of the exe data the code reads: constants and globals.
        e.map(0x0101_0000, 0x11000);
        e.map(0x0104_a000, 0x1000);
        e.map(0x0119_9000, 0x2000);
        e.map(0x011c_0000, 0x30000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e
    }

    /// The addresses of the doubles' calls, in order.
    fn started(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn calls(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn new_npc(e: &mut Engine) -> Ptr<TESNPC> {
        let npc: Ptr<TESNPC> = e.new_object();
        e.set(npc, TESNPC::cFormType, 0x2a);
        npc
    }

    /// A small object with a vtable whose slots (by byte offset) go to the
    /// given addresses, at a fresh vtable address.
    fn put_object_vtable(e: &mut Engine, object: u32, table: u32, slots: &[(u32, u32)]) {
        let size = slots.iter().map(|s| s.0).max().unwrap_or(0) + 4;
        e.map(table, size);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        e.mem.set_u32(object, table);
    }

    /// A text in game memory.
    fn text(e: &mut Engine, s: &str) -> u32 {
        let block = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(block, s.as_bytes());
        block
    }

    #[test]
    fn constructor_builds_the_bases_stores_the_vtables_and_initializes() {
        let mut e = engine();
        started(&mut e);
        let this: Ptr<TESNPC> = e.new_object();
        let base = this.addr();
        let result = e.call(0x0060_1170, &args![this]).ptr::<TESNPC>();
        assert_eq!(result, this);
        for (offset, vtable) in VTABLE_STORES {
            assert_eq!(e.mem.u32(base + offset), vtable);
        }
        assert_eq!(calls(&mut e, 0x005f_75e0), vec![vec![base]]);
        assert_eq!(calls(&mut e, 0x0048_b800), vec![vec![base + 0x10c]]);
        assert_eq!(
            calls(&mut e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![base + 0x134, 0x20, 4, 0x0044_9610, 0x0044_9680]]
        );
        assert_eq!(
            calls(&mut e, NI_POINTER_CONSTRUCT),
            vec![
                vec![base + 0x1c4, 0],
                vec![base + 0x1c8, 0],
                vec![base + 0x1cc, 0]
            ]
        );
        assert_eq!(calls(&mut e, 0x0096_a2d0), vec![vec![base + 0x1dc]]);
        assert_eq!(calls(&mut e, 0x0060_b9e0), vec![vec![base + 0x1fc, 0, 1]]);
        assert_eq!(calls(&mut e, 0x004f_15a0), vec![vec![base, 0x2a]]);
        let ai = base + COMPONENT_AI_FORM;
        assert_eq!(calls(&mut e, 0x0047_ef50), vec![vec![ai, 0x32, 0]]);
        assert_eq!(calls(&mut e, 0x0047_ee00), vec![vec![ai, 0, 0]]);
        assert_eq!(calls(&mut e, 0x0047_eed0), vec![vec![ai, 2, 0]]);
        assert_eq!(calls(&mut e, 0x0047_ef00), vec![vec![ai, 0x32, 0]]);
        // InitializeData ran in between.
        assert_eq!(e.get(this, TESNPC::sLastRaceFaceNum), 0xff);
        assert_eq!(e.get(this, TESNPC::iFileOffset), 0);
    }

    #[test]
    fn initialize_data_resets_the_fields_and_copies_the_default_coordinate() {
        let mut e = engine();
        let this = new_npc(&mut e);
        let base = this.addr();
        e.set(this, TESNPC::fWeight, 5.0);
        e.set(this, TESNPC::fHeight, 6.0);
        e.set(this, TESNPC::pHair, Ptr::new(1));
        e.set(this, TESNPC::fHairLength, 2.0);
        e.set(this, TESNPC::pEyeColor, Ptr::new(3));
        e.set(this, TESNPC::pCl, Ptr::new(4));
        e.set(this, TESNPC::pCombatStyle, Ptr::new(5));
        e.set(this, TESNPC::iHairColor, 6);
        e.set(this, TESNPC::pAlternateFaceOffsetCoord, Ptr::new(0x7777));
        e.set(this, TESNPC::pOriginalRace, Ptr::new(8));
        e.set(this, TESNPC::pFaceNPC, Ptr::new(9));
        e.set(this, TESNPC::eBloodImpactMaterial, 1);
        e.register(FACEGEN_DEFAULT_COORD, |_, _| ret(0x5000));
        // Only the second and third matrices of the race coordinate hold data.
        e.register_double(0x0096_11e0, move |_, a| {
            ret((a[0] == base + 0x134 + 0x20 || a[0] == base + 0x134 + 0x40) as u32)
        });
        started(&mut e);
        e.call(0x0060_1570, &args![this]);
        assert_eq!(e.get(this, TESNPC::fWeight), 0.0);
        assert_eq!(e.get(this, TESNPC::fHeight), 0.0);
        assert_eq!(e.get(this, TESNPC::pHair), Ptr::NULL);
        assert_eq!(e.get(this, TESNPC::fHairLength), 0.0);
        assert_eq!(e.get(this, TESNPC::pEyeColor), Ptr::NULL);
        assert_eq!(e.get(this, TESNPC::sLastRaceFaceNum), 0xff);
        assert_eq!(e.get(this, TESNPC::pCl), Ptr::NULL);
        assert_eq!(e.get(this, TESNPC::pCombatStyle), Ptr::NULL);
        assert_eq!(e.get(this, TESNPC::iHairColor), 0x0019_324b);
        assert_eq!(e.get(this, TESNPC::pAlternateFaceOffsetCoord), Ptr::NULL);
        assert_eq!(e.get(this, TESNPC::eBloodImpactMaterial), 6);
        assert_eq!(e.get(this, TESNPC::pOriginalRace), Ptr::NULL);
        assert_eq!(e.get(this, TESNPC::pFaceNPC), Ptr::NULL);
        for k in 0..14 {
            assert_eq!(e.mem.u8(base + 0x114 + k), 5);
            assert_eq!(e.mem.u8(base + 0x122 + k), 0);
        }
        // The default coordinate is copied into the NPC's own (the alternate
        // pointer was just cleared, so the race coordinate).
        assert_eq!(
            calls(&mut e, FACEGEN_COPY_COORD),
            vec![vec![0x5000, base + 0x134, 0, 0]]
        );
        assert_eq!(
            calls(&mut e, 0x0060_b3b0),
            vec![vec![base + 0x134 + 0x20], vec![base + 0x134 + 0x40]]
        );
        assert_eq!(
            calls(&mut e, 0x005f_7b50),
            vec![vec![base + COMPONENT_ACTOR_BASE_DATA, 1, 0]]
        );
        assert_eq!(
            calls(&mut e, SET_WORD_AT_4),
            vec![vec![base + COMPONENT_HEALTH, 0x32]]
        );
        // Without a save-load object that reports otherwise, the head
        // pointers are cleared.
        assert_eq!(
            calls(&mut e, NI_POINTER_ASSIGN),
            vec![vec![base + 0x1c8, 0], vec![base + 0x1cc, 0]]
        );
        assert_eq!(
            calls(&mut e, NI_POINTER_ASSIGN_POINTER),
            vec![vec![base + 0x1c4, base + 0x1c8]]
        );
    }

    #[test]
    fn initialize_data_leaves_the_head_pointers_when_the_save_load_object_says_so() {
        let mut e = engine();
        let this = new_npc(&mut e);
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| ret(1));
        started(&mut e);
        tesnpc_initialize_data(&mut e, this);
        assert!(calls(&mut e, NI_POINTER_ASSIGN).is_empty());
        assert!(calls(&mut e, NI_POINTER_ASSIGN_POINTER).is_empty());
    }

    #[test]
    fn set_hair_and_set_eye_color_store_the_pointers() {
        let mut e = engine();
        let this = new_npc(&mut e);
        e.call(0x0060_31e0, &args![this, 0x1111u32]);
        e.call(0x0060_3200, &args![this, 0x2222u32]);
        assert_eq!(e.get(this, TESNPC::pHair), Ptr::new(0x1111));
        assert_eq!(e.get(this, TESNPC::pEyeColor), Ptr::new(0x2222));
    }

    /// Two forms whose full names are the strings given.
    fn forms_named(e: &mut Engine, names: [&str; 2]) -> [Ptr; 2] {
        // `TESFullName` at +0xd0; its string getter returns the character
        // pointer stored at +4 of the component, or the empty string.
        e.register(0x0040_8da0, |e, a| {
            let string = e.mem.u32(a[0] + 4);
            ret(if string != 0 { string } else { 0x0101_1584 })
        });
        e.mem.set_cstr(0x0101_1584, b"");
        let mut forms = [Ptr::NULL; 2];
        for (i, name) in names.iter().enumerate() {
            let form = Ptr::new(e.mem.alloc(0x100));
            let string = if name.is_empty() { 0 } else { text(e, name) };
            e.mem.set_u32(form.addr() + COMPONENT_FULL_NAME + 4, string);
            forms[i] = form;
        }
        forms
    }

    #[test]
    fn name_comparison_orders_forms_by_their_full_names() {
        let mut e = engine();
        let [apple, banana] = forms_named(&mut e, ["apple", "banana"]);
        let [nameless, _] = forms_named(&mut e, ["", "x"]);
        let slot = |e: &mut Engine, form: Ptr| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, form.addr());
            Ptr::<()>::new(slot)
        };
        let a = slot(&mut e, apple);
        let b = slot(&mut e, banana);
        let n = slot(&mut e, nameless);
        let empty = slot(&mut e, Ptr::NULL);
        let compare = |e: &mut Engine, x: Ptr, y: Ptr| e.call(0x0060_3220, &args![x, y]).i32();
        assert_eq!(compare(&mut e, a, b), -1);
        assert_eq!(compare(&mut e, b, a), 1);
        assert_eq!(compare(&mut e, a, a), 0);
        // A nameless form compares as the empty string.
        assert_eq!(compare(&mut e, n, a), -1);
        // A null pointer or a pointer to null compares as equal.
        assert_eq!(compare(&mut e, Ptr::NULL, a), 0);
        assert_eq!(compare(&mut e, a, Ptr::NULL), 0);
        assert_eq!(compare(&mut e, empty, a), 0);
        assert_eq!(compare(&mut e, a, empty), 0);
    }

    type Collected = Rc<RefCell<Vec<u32>>>;

    /// A world with a data handler, a player and a chain of NPC forms
    /// (`(race, flags)`), linked through `+0x20`. Returns the list array,
    /// the NPCs and what the append double collected.
    fn npc_world(e: &mut Engine, npcs: &[(u32, u32)]) -> (Ptr, Vec<Ptr<TESNPC>>, Collected) {
        let handler = e.mem.alloc(0x100);
        let objects = e.mem.alloc(0x10);
        e.set_global(DATA_HANDLER, handler);
        e.mem.set_u32(handler + 4, objects);
        let mut chain = vec![];
        for (race, flags) in npcs {
            let npc = new_npc(e);
            e.set(npc, TESNPC::pFormRace, Ptr::new(*race));
            e.set(npc, TESNPC::iActorBaseFlags, *flags);
            chain.push(npc);
        }
        for pair in chain.windows(2) {
            e.mem.set_u32(pair[0].addr() + 0x20, pair[1].addr());
        }
        e.mem
            .set_u32(objects + 4, chain.first().map_or(0, |n| n.addr()));
        // The player's reference: its base form is the last NPC.
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER_SINGLETON, player);
        e.mem
            .set_u32(player + 0x20, chain.last().map_or(0, |n| n.addr()));
        let list = Ptr::new(e.mem.alloc(0x10));
        let collected = Rc::new(RefCell::new(vec![]));
        let sink = collected.clone();
        e.register_double(0x007c_b2e0, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[1]));
            ret(0)
        });
        let seen = collected.clone();
        e.register_double(
            0x0076_b610,
            move |_, _| ret(seen.borrow().is_empty() as u32),
        );
        (list, chain, collected)
    }

    #[test]
    fn preset_collection_takes_matching_npcs_with_the_flag_and_sorts_them() {
        let mut e = engine();
        // Races 7, 7, 8, 7 (the last one is the player's base form); the
        // second NPC is female; only the first and third have flag 4.
        let (list, npcs, collected) = npc_world(&mut e, &[(7, 4), (7, 1), (8, 4), (7, 4)]);
        started(&mut e);
        e.call(0x0060_3280, &args![Ptr::<()>::new(7), 0u32, list]);
        assert_eq!(*collected.borrow(), vec![npcs[0].addr()]);
        assert_eq!(
            calls(&mut e, 0x0072_9970),
            vec![vec![list.addr(), 0x0060_3220]]
        );
        // The female with the same race has no flag 4, so the first pass finds
        // nothing and the second pass (without the flag test) finds her.
        collected.borrow_mut().clear();
        e.call(0x0060_3280, &args![Ptr::<()>::new(7), 1u32, list]);
        assert_eq!(*collected.borrow(), vec![npcs[1].addr()]);
    }

    #[test]
    fn preset_collection_retries_without_the_flag_and_skips_a_null_list() {
        let mut e = engine();
        let (list, npcs, collected) = npc_world(&mut e, &[(7, 0), (7, 0), (9, 0)]);
        started(&mut e);
        e.call(0x0060_3280, &args![Ptr::<()>::new(7), 0u32, list]);
        assert_eq!(*collected.borrow(), vec![npcs[0].addr(), npcs[1].addr()]);
        // A null list does nothing at all.
        collected.borrow_mut().clear();
        e.call_log = Some(vec![]);
        e.call(
            0x0060_3280,
            &args![Ptr::<()>::new(7), 0u32, Ptr::<()>::NULL],
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert!(collected.borrow().is_empty());
    }

    /// Fake virtual-table addresses for the doubles of virtual calls.
    const FAKE_VTABLE: u32 = 0x0ba0_0000;
    const FAKE_COMPONENT_VTABLE: u32 = 0x0ba1_0000;
    const FAKE_AUTO_CALC: u32 = 0x0ba2_0001;
    const FAKE_NOOP: u32 = 0x0ba2_0007;
    const FAKE_GET_COMBAT_STYLE: u32 = 0x0ba2_0002;
    const FAKE_SET_COMBAT_STYLE: u32 = 0x0ba2_0003;
    const FAKE_GET_NAME: u32 = 0x0ba2_0004;
    const FAKE_GET_BLOOD: u32 = 0x0ba2_0005;
    const FAKE_SET_BLOOD: u32 = 0x0ba2_0006;

    /// Gives an NPC (and its actor-base component) a vtable whose combat
    /// style getter returns the `pCombatStyle` field, whose setter stores
    /// it, whose name getter returns a fixed string and whose auto-calc
    /// getter returns `auto_calc`; the blood-impact slots of the component
    /// read and write `eBloodImpactMaterial`.
    fn give_vtables(e: &mut Engine, npc: Ptr<TESNPC>, auto_calc: bool, index: u32) {
        put_object_vtable(
            e,
            npc.addr(),
            FAKE_VTABLE + index * 0x1000,
            &[
                (0x48, FAKE_NOOP),
                (0x144, FAKE_AUTO_CALC),
                (0x148, FAKE_NOOP),
                (0x188, FAKE_GET_COMBAT_STYLE),
                (0x18c, FAKE_SET_COMBAT_STYLE),
                (0x130, FAKE_GET_NAME),
                (0xe0, FAKE_NOOP),
                (0x134, FAKE_NOOP),
            ],
        );
        put_object_vtable(
            e,
            npc.addr() + COMPONENT_ACTOR_BASE_DATA,
            FAKE_COMPONENT_VTABLE + index * 0x1000,
            &[
                (0x10, FAKE_NOOP),
                (0x2c, FAKE_NOOP),
                (0x30, FAKE_NOOP),
                (0x58, FAKE_GET_BLOOD),
                (0x5c, FAKE_SET_BLOOD),
                (0x60, FAKE_NOOP),
                (0x64, FAKE_NOOP),
                (0x68, FAKE_NOOP),
            ],
        );
        e.register(FAKE_NOOP, |_, _| ret(0));
        e.register_double(FAKE_AUTO_CALC, move |_, _| ret(auto_calc as u32));
        e.register(FAKE_GET_COMBAT_STYLE, |e, a| ret(e.mem.u32(a[0] + 0x1d4)));
        e.register(FAKE_SET_COMBAT_STYLE, |e, a| {
            e.mem.set_u32(a[0] + 0x1d4, a[1]);
            ret(0)
        });
        e.register(FAKE_GET_NAME, |_, _| ret(0x0777_0000));
        e.register(FAKE_GET_BLOOD, |e, a| ret(e.mem.u32(a[0] - 0x30 + 0x1e4)));
        e.register(FAKE_SET_BLOOD, |e, a| {
            e.mem.set_u32(a[0] - 0x30 + 0x1e4, a[1]);
            ret(0)
        });
    }

    #[test]
    fn init_item_does_nothing_for_a_form_that_is_already_initialized() {
        let mut e = engine();
        let this = new_npc(&mut e);
        e.register(0x0040_13e0, |_, _| ret(1));
        started(&mut e);
        e.call(0x0060_3370, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn init_item_initializes_components_and_resolves_class_and_combat_style() {
        let mut e = engine();
        let this = new_npc(&mut e);
        let base = this.addr();
        give_vtables(&mut e, this, true, 0);
        e.set(this, TESNPC::iActorBaseFlags, 4);
        e.set(this, TESNPC::pCl, Ptr::new(0x1234));
        e.set(this, TESNPC::pCombatStyle, Ptr::new(0x4321));
        // The file index is fixed up into the id, the lookup finds forms.
        e.register(0x0048_4e60, |_, _| ret(0x5555));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            ret(0)
        });
        e.register(0x0048_39c0, |_, a| {
            // The class form exists, the combat style form does not.
            ret(if a[0] == 0x0100_1234 { 0x7000 } else { 0 })
        });
        e.register(0x0050_5000, |_, _| ret(0x9999));
        e.set(this, TESNPC::iFormID, 0x0100_0abc);
        // Health 0, height 0: the race height for a male is used.
        let race = e.mem.alloc(0x100);
        e.mem.set_f32(race + 0x60, 1.75);
        e.set(this, TESNPC::pFormRace, Ptr::new(race));
        started(&mut e);
        e.call(0x0060_3370, &args![this]);

        // The preset list got the NPC.
        let added = calls(&mut e, 0x005a_e3d0);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], CHARGEN_PRESET_FACE_LIST);
        // Each component's InitItem got the NPC.
        let component_calls = [
            (0x0048_cd90, COMPONENT_SCRIPTABLE),
            (0x0048_1bb0, COMPONENT_CONTAINER),
            (0x0047_f4c0, COMPONENT_AI_FORM),
            (0x0048_b920, COMPONENT_RACE),
            (0x0047_d8b0, COMPONENT_ACTOR_BASE_DATA),
            (0x0048_d400, COMPONENT_SPELL_LIST),
            (0x0047_beb0, COMPONENT_TOUCH_SPELL),
            (0x0047_8be0, COMPONENT_DESTRUCTIBLE),
        ];
        for (addr, offset) in component_calls {
            assert_eq!(calls(&mut e, addr), vec![vec![base + offset, base]]);
        }
        // The class resolves; the combat style is reported and replaced.
        assert_eq!(e.get(this, TESNPC::pCl), Ptr::new(0x7000));
        let report = calls(&mut e, 0x005b_5e40);
        assert_eq!(
            report,
            vec![vec![0x0104_a600, 0x0100_4321, 0x0100_0abc, 0x0777_0000]]
        );
        assert_eq!(e.get(this, TESNPC::pCombatStyle), Ptr::new(0x9999));
        // Auto-calc: InitValues ran (it needs a race and a class; the race
        // exists, the class does).
        assert_eq!(calls(&mut e, 0x0056_5210), vec![vec![base, 1]]);
        assert_eq!(e.get(this, TESNPC::fHeight), 1.75);
        assert_eq!(calls(&mut e, 0x0048_4ab0), vec![vec![base, 1]]);
    }

    #[test]
    fn init_item_keeps_a_valid_combat_style_and_a_set_height() {
        let mut e = engine();
        let this = new_npc(&mut e);
        give_vtables(&mut e, this, false, 0);
        e.set(this, TESNPC::pCombatStyle, Ptr::new(0x4321));
        e.set(this, TESNPC::fHeight, 2.0);
        e.set(this, TESNPC::iHealth, 40);
        e.register(0x0048_39c0, |_, _| ret(0x6000));
        started(&mut e);
        e.call(0x0060_3370, &args![this]);
        assert_eq!(e.get(this, TESNPC::pCombatStyle), Ptr::new(0x6000));
        assert!(calls(&mut e, 0x005b_5e40).is_empty());
        assert!(calls(&mut e, 0x0056_5210).is_empty());
        assert_eq!(e.get(this, TESNPC::fHeight), 2.0);
        // Not an auto-calc NPC: no class was looked up.
        assert_eq!(e.get(this, TESNPC::pCl), Ptr::NULL);
        assert_eq!(calls(&mut e, 0x0048_39c0).len(), 1);
    }

    #[test]
    fn copy_ignores_forms_that_are_not_npcs_and_copies_every_part_of_an_npc() {
        let mut e = engine();
        let this = new_npc(&mut e);
        let source = new_npc(&mut e);
        let base = this.addr();
        let from = source.addr();
        give_vtables(&mut e, this, false, 0);
        give_vtables(&mut e, source, false, 1);
        e.set(source, TESNPC::pCl, Ptr::new(0x77));
        e.set(source, TESNPC::pCombatStyle, Ptr::new(0x88));
        e.set(source, TESNPC::eBloodImpactMaterial, 3);
        e.set(source, TESNPC::fHeight, 1.8);
        e.set(source, TESNPC::fWeight, 82.5);
        for i in 0..0x1c {
            e.mem.set_u8(from + 0x114 + i, i as u8 + 1);
        }

        // A form of another type: nothing is copied.
        let other = new_npc(&mut e);
        e.set(other, TESNPC::cFormType, 0x10);
        started(&mut e);
        e.call(0x0060_35e0, &args![this, other]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);

        e.call(0x0060_35e0, &args![this, source]);
        let copies = [
            (0x0047_f190, COMPONENT_AI_FORM),
            (0x0048_d630, COMPONENT_SPELL_LIST),
            (0x0047_bdd0, COMPONENT_TOUCH_SPELL),
            (0x0048_b870, COMPONENT_RACE),
            (0x0047_d220, COMPONENT_ACTOR_BASE_DATA),
            (0x0048_7240, COMPONENT_HEALTH),
            (0x0048_0000, COMPONENT_ATTRIBUTES),
            (0x0048_70f0, COMPONENT_FULL_NAME),
            (0x0048_9430, COMPONENT_MODEL),
            (0x0048_cce0, COMPONENT_SCRIPTABLE),
            (0x0048_1c80, COMPONENT_CONTAINER),
            (0x0047_fb50, COMPONENT_ANIMATION),
            (0x0047_86e0, COMPONENT_DESTRUCTIBLE),
        ];
        for (addr, offset) in copies {
            assert_eq!(calls(&mut e, addr), vec![vec![base + offset, from]]);
        }
        assert_eq!(
            e.mem.bytes(base + 0x114, 0x1c),
            e.mem.bytes(from + 0x114, 0x1c)
        );
        assert_eq!(e.get(this, TESNPC::pCl), Ptr::new(0x77));
        assert_eq!(e.get(this, TESNPC::pCombatStyle), Ptr::new(0x88));
        assert_eq!(e.get(this, TESNPC::eBloodImpactMaterial), 3);
        assert_eq!(e.get(this, TESNPC::fHeight), 1.8);
        assert_eq!(e.get(this, TESNPC::fWeight), 82.5);
        // The head copy ran (its face NPC link is set).
        assert_eq!(calls(&mut e, 0x004c_0cd0), vec![vec![base, from]]);
    }

    #[test]
    fn head_copy_takes_hair_eyes_color_and_head_parts() {
        let mut e = engine();
        let this = new_npc(&mut e);
        let source = new_npc(&mut e);
        let base = this.addr();
        let from = source.addr();
        e.register(0x004c_0cd0, |e, a| {
            e.mem.set_u32(a[0] + 0x1f0, a[1]);
            ret(0)
        });
        e.set(source, TESNPC::pHair, Ptr::new(0xa1));
        e.set(source, TESNPC::fHairLength, 0.75);
        e.set(source, TESNPC::pEyeColor, Ptr::new(0xe1));
        e.set(source, TESNPC::iHairColor, 0x00ff_00ff);
        // Two head parts in the source's list: the node embedded at +0x1dc
        // and one more; both items are copied, in order.
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0xb2);
        e.mem.set_u32(from + 0x1dc, 0xb1);
        e.mem.set_u32(from + 0x1dc + 4, second);
        started(&mut e);
        e.call(0x0060_3790, &args![this, source]);
        assert_eq!(e.get(this, TESNPC::pHair), Ptr::new(0xa1));
        assert_eq!(e.get(this, TESNPC::fHairLength), 0.75);
        assert_eq!(e.get(this, TESNPC::pEyeColor), Ptr::new(0xe1));
        assert_eq!(e.get(this, TESNPC::iHairColor), 0x00ff_00ff);
        assert_eq!(calls(&mut e, 0x0047_0470), vec![vec![base + 0x1dc]]);
        // The appends receive the address of each node (its item slot).
        assert_eq!(
            calls(&mut e, 0x0090_5820),
            vec![vec![base + 0x1dc, from + 0x1dc], vec![base + 0x1dc, second],]
        );
        assert_eq!(
            calls(&mut e, FACEGEN_COPY_COORD),
            vec![vec![from + 0x134, base + 0x134, 0, 0]]
        );
        // No player: the face NPC becomes the source.
        assert_eq!(e.get(this, TESNPC::pFaceNPC), Ptr::new(from));

        // The player's base form keeps its own face NPC.
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER_SINGLETON, player);
        e.mem.set_u32(player + 0x20, base);
        e.set(this, TESNPC::pFaceNPC, Ptr::NULL);
        e.call(0x0060_3790, &args![this, source]);
        assert_eq!(e.get(this, TESNPC::pFaceNPC), Ptr::NULL);
    }

    #[test]
    fn hair_of_an_npc_with_an_original_race_comes_from_the_race() {
        let mut e = engine();
        let this = new_npc(&mut e);
        e.set(this, TESNPC::pHair, Ptr::new(0xa1));
        e.register(0x0061_3870, |_, a| ret(0xfeed_0000 + a[1]));
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER_SINGLETON, player);
        // No original race: the stored hair.
        assert_eq!(e.call(0x0060_3b50, &args![this]).u32(), 0xa1);
        // With one: the race's default for the NPC's sex.
        e.set(this, TESNPC::pOriginalRace, Ptr::new(0x55));
        e.set(this, TESNPC::iActorBaseFlags, 1);
        e.set(this, TESNPC::pFormRace, Ptr::new(0x66));
        assert_eq!(e.call(0x0060_3b50, &args![this]).u32(), 0xfeed_0001);
        // The player's base form always keeps its own.
        e.mem.set_u32(player + 0x20, this.addr());
        assert_eq!(e.call(0x0060_3b50, &args![this]).u32(), 0xa1);
    }

    #[test]
    fn head_pointers_are_cleared() {
        let mut e = engine();
        let this = new_npc(&mut e);
        let base = this.addr();
        started(&mut e);
        e.call(0x0060_3bb0, &args![this]);
        assert_eq!(
            calls(&mut e, NI_POINTER_ASSIGN),
            vec![vec![base + 0x1c4, 0], vec![base + 0x1c8, 0]]
        );
    }

    #[test]
    fn face_coordinate_is_the_default_without_a_race_and_blended_with_one() {
        let mut e = engine();
        let this = new_npc(&mut e);
        let base = this.addr();
        e.register(FACEGEN_DEFAULT_COORD, |_, _| ret(0x5000));
        started(&mut e);
        e.call(0x0060_3ad0, &args![this, 0x9000u32]);
        assert_eq!(calls(&mut e, FACEGEN_INIT_COORD), vec![vec![0x9000]]);
        assert_eq!(
            calls(&mut e, FACEGEN_COPY_COORD),
            vec![vec![0x5000, 0x9000, 0, 0]]
        );
        assert!(calls(&mut e, FACEGEN_BLEND_COORDS).is_empty());

        // A female NPC with a race: the race's coordinate for her sex.
        e.set(this, TESNPC::pFormRace, Ptr::new(0x6000));
        e.set(this, TESNPC::iActorBaseFlags, 1);
        e.register(0x005d_9fb0, |_, a| ret(0x7000 + a[1]));
        e.call_log = Some(vec![]);
        e.call(0x0060_3ad0, &args![this, 0x9000u32]);
        assert_eq!(
            calls(&mut e, FACEGEN_BLEND_COORDS),
            vec![vec![0x7001, base + 0x134, 0x9000, 0, 0]]
        );
        assert!(calls(&mut e, FACEGEN_COPY_COORD).is_empty());
    }

    /// An engine for comparing two NPCs: the list helpers get their real
    /// bodies (count the non-empty nodes; find an item), `memcmp` compares
    /// bytes.
    fn compare_world() -> (Engine, Ptr<TESNPC>, Ptr<TESNPC>) {
        let mut e = engine();
        let a = new_npc(&mut e);
        let b = new_npc(&mut e);
        give_vtables(&mut e, a, false, 0);
        give_vtables(&mut e, b, false, 1);
        e.register(MEMCMP, |e, a| {
            let first = e.mem.bytes(a[0], a[2]);
            let second = e.mem.bytes(a[1], a[2]);
            ret(match first.cmp(&second) {
                std::cmp::Ordering::Less => u32::MAX,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            })
        });
        e.register(0x005a_e380, |e, a| {
            let mut count = 0;
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    count += 1;
                }
                node = e.mem.u32(node + 4);
            }
            ret(count)
        });
        e.register(0x005f_65d0, |e, a| {
            let wanted = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 && e.mem.u32(node) != wanted {
                node = e.mem.u32(node + 4);
            }
            ret((node != 0) as u32)
        });
        // The same single head part in both lists.
        e.mem.set_u32(a.addr() + 0x1dc, 0xb1);
        e.mem.set_u32(b.addr() + 0x1dc, 0xb1);
        (e, a, b)
    }

    fn compare(e: &mut Engine, a: Ptr<TESNPC>, b: Ptr<TESNPC>) -> bool {
        e.call(0x0060_3880, &args![a, b]).bool()
    }

    #[test]
    fn compare_finds_equal_npcs_equal_and_each_difference() {
        let (mut e, a, b) = compare_world();
        assert!(!compare(&mut e, a, b));
        type Change = Box<dyn Fn(&mut Engine, Ptr<TESNPC>)>;
        let differences: Vec<(&str, Change)> = vec![
            (
                "data",
                Box::new(|e, n| e.mem.set_u8(n.addr() + 0x114 + 5, 9)),
            ),
            ("class", Box::new(|e, n| e.set(n, TESNPC::pCl, Ptr::new(3)))),
            (
                "hair",
                Box::new(|e, n| e.set(n, TESNPC::pHair, Ptr::new(3))),
            ),
            (
                "hair length",
                Box::new(|e, n| e.set(n, TESNPC::fHairLength, 0.5)),
            ),
            (
                "eyes",
                Box::new(|e, n| e.set(n, TESNPC::pEyeColor, Ptr::new(3))),
            ),
            (
                "hair color",
                Box::new(|e, n| e.set(n, TESNPC::iHairColor, 3)),
            ),
            (
                "combat style",
                Box::new(|e, n| e.set(n, TESNPC::pCombatStyle, Ptr::new(3))),
            ),
            (
                "blood",
                Box::new(|e, n| e.set(n, TESNPC::eBloodImpactMaterial, 3)),
            ),
            ("height", Box::new(|e, n| e.set(n, TESNPC::fHeight, 3.0))),
            ("weight", Box::new(|e, n| e.set(n, TESNPC::fWeight, 3.0))),
            (
                "head part",
                Box::new(|e, n| e.mem.set_u32(n.addr() + 0x1dc, 0xb2)),
            ),
            (
                "head part count",
                Box::new(|e, n| {
                    let more = e.mem.alloc(8);
                    e.mem.set_u32(more, 0xb3);
                    e.mem.set_u32(n.addr() + 0x1dc + 4, more);
                }),
            ),
        ];
        for (what, change) in differences {
            let (mut e, a, b) = compare_world();
            change(&mut e, b);
            assert!(compare(&mut e, a, b), "{what}");
        }
        // The components, the face coordinate and a form that is not an NPC.
        let (mut e, a, b) = compare_world();
        e.register(0x0048_5270, |_, _| ret(1));
        assert!(compare(&mut e, a, b));
        let (mut e, a, b) = compare_world();
        e.register(FACEGEN_COORDS_DIFFER, |_, _| ret(1));
        assert!(compare(&mut e, a, b));
        let (mut e, a, b) = compare_world();
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(compare(&mut e, a, b));
    }

    #[test]
    fn form_health_is_the_stored_value_but_never_negative() {
        let mut e = engine();
        let component = e.mem.alloc(8);
        e.mem.set_i32(component + 4, 25);
        assert_eq!(e.call(0x0060_3f90, &args![component]).i32(), 25);
        e.mem.set_i32(component + 4, -7);
        assert_eq!(e.call(0x0060_3f90, &args![component]).i32(), 0);
    }

    /// An NPC with the fixed numbers the health and fatigue formulas read.
    fn formula_npc(e: &mut Engine) -> Ptr<TESNPC> {
        let this = new_npc(e);
        give_vtables(e, this, false, 0);
        e.mem
            .set_u16(this.addr() + COMPONENT_ACTOR_BASE_DATA + 8, 4);
        e.register(ATTRIBUTES_GET_SEVENTH, |_, _| ret(6));
        // Float settings (value at +4 of each object).
        for (setting, value) in [
            (0x011c_d27cu32, 2.0f32),
            (0x011c_d544, 3.0),
            (0x011c_d908, 4.0),
            (0x011c_d974, 7.5),
            (0x011c_e018, 2.0),
            (0x011c_db54, 1.5),
        ] {
            e.mem.set_f32(setting + 4, value);
        }
        this
    }

    #[test]
    fn health_formula_uses_level_attribute_and_optionally_the_stored_health() {
        let mut e = engine();
        let this = formula_npc(&mut e);
        e.set(this, TESNPC::iHealth, 50);
        // base 50: 4 * (4 - 1) + (3 * (2 + 6) + 50) = 86.
        assert_eq!(e.call(0x0060_3fc0, &args![this, 1u32]).i32(), 86);
        // without it: 12 + 24.
        assert_eq!(e.call(0x0060_3fc0, &args![this, 0u32]).i32(), 36);
        // A level below 1 counts as 1.
        e.mem
            .set_u16(this.addr() + COMPONENT_ACTOR_BASE_DATA + 8, 0);
        assert_eq!(e.call(0x0060_3fc0, &args![this, 1u32]).i32(), 74);
        // No stored health: zero. A negative result: zero.
        e.set(this, TESNPC::iHealth, 0);
        assert_eq!(e.call(0x0060_3fc0, &args![this, 1u32]).i32(), 0);
        e.set(this, TESNPC::iHealth, 50);
        e.mem.set_f32(0x011c_d908 + 4, -1000.0);
        e.mem
            .set_u16(this.addr() + COMPONENT_ACTOR_BASE_DATA + 8, 9);
        assert_eq!(e.call(0x0060_3fc0, &args![this, 1u32]).i32(), 0);
    }

    #[test]
    fn fatigue_formula_uses_the_fatigue_or_its_default() {
        let mut e = engine();
        let this = formula_npc(&mut e);
        // The component's virtual 0x60 is the fatigue.
        e.mem.set_u32(FAKE_COMPONENT_VTABLE + 0x60, 0x0ba2_0010);
        e.register(0x0ba2_0010, |_, _| ret(30));
        // 1.5 * 4 + (2 * 6 + 30) = 48.
        assert_eq!(e.call(0x0060_40d0, &args![this, 1u32]).i32(), 48);
        // The default 7.5 instead: 6 + 19.5 = 25.5, truncated.
        assert_eq!(e.call(0x0060_40d0, &args![this, 0u32]).i32(), 25);
        // No fatigue: zero.
        e.register(0x0ba2_0010, |_, _| ret(0));
        assert_eq!(e.call(0x0060_40d0, &args![this, 1u32]).i32(), 0);
    }

    #[test]
    fn global_getter_returns_the_exe_word() {
        let mut e = engine();
        e.set_global(0x011c_6210, 0x1234_5678u32);
        assert_eq!(e.call(0x0060_41c0, &args![]).u32(), 0x1234_5678);
    }

    #[test]
    fn race_height_and_weight_depend_on_the_sex() {
        let mut e = engine();
        let this = new_npc(&mut e);
        let race = e.mem.alloc(0x100);
        e.mem.set_f32(race + 0x60, 1.8);
        e.mem.set_f32(race + 0x64, 1.6);
        e.mem.set_f32(race + 0x68, 70.0);
        e.mem.set_f32(race + 0x6c, 60.0);
        // No race: 0.0.
        assert_eq!(e.call(0x0060_41d0, &args![this]).f32(), 0.0);
        e.set(this, TESNPC::pFormRace, Ptr::new(race));
        assert_eq!(e.call(0x0060_41d0, &args![this]).f32(), 1.8);
        e.set(this, TESNPC::iActorBaseFlags, 1);
        assert_eq!(e.call(0x0060_41d0, &args![this]).f32(), 1.6);
        // The per-race tables themselves: only sex 0 and 1.
        assert_eq!(e.call(0x0060_4210, &args![race, 0u32]).f32(), 1.8);
        assert_eq!(e.call(0x0060_4210, &args![race, 2u32]).f32(), 0.0);
        assert_eq!(e.call(0x0060_4210, &args![race, u32::MAX]).f32(), 0.0);
        assert_eq!(e.call(0x0060_42d0, &args![race, 0u32]).f32(), 70.0);
        assert_eq!(e.call(0x0060_42d0, &args![race, 1u32]).f32(), 60.0);
        assert_eq!(e.call(0x0060_42d0, &args![race, 2u32]).f32(), 0.0);
        assert_eq!(e.call(0x0060_42d0, &args![race, u32::MAX]).f32(), 0.0);
    }

    #[test]
    fn weight_is_special_for_the_player_base_and_odd_for_everyone_else() {
        let mut e = engine();
        let this = new_npc(&mut e);
        e.set(this, TESNPC::iFormID, 0x55);
        e.set(this, TESNPC::fHeight, 1.9);
        e.set(this, TESNPC::fWeight, 77.0);
        // Another NPC: the exe returns the float at +0x1f4 (the height field).
        assert_eq!(e.call(0x0060_4250, &args![this]).f32(), 1.9);
        // The player's base form (form id 7): its stored weight...
        e.set(this, TESNPC::iFormID, 7);
        assert_eq!(e.call(0x0060_4250, &args![this]).f32(), 77.0);
        // ...or, when that is 0.0, the race's weight for its sex.
        e.set(this, TESNPC::fWeight, 0.0);
        assert_eq!(e.call(0x0060_4250, &args![this]).f32(), 0.0);
        let race = e.mem.alloc(0x100);
        e.mem.set_f32(race + 0x68, 70.0);
        e.mem.set_f32(race + 0x6c, 60.0);
        e.set(this, TESNPC::pFormRace, Ptr::new(race));
        assert_eq!(e.call(0x0060_4250, &args![this]).f32(), 70.0);
        e.set(this, TESNPC::iActorBaseFlags, 1);
        assert_eq!(e.call(0x0060_4250, &args![this]).f32(), 60.0);
    }

    #[test]
    fn weight_setter_stores_the_float() {
        let mut e = engine();
        let this = new_npc(&mut e);
        e.call(0x0060_53f0, &args![this, 63.5f32]);
        assert_eq!(e.get(this, TESNPC::fWeight), 63.5);
    }

    #[test]
    fn face_tester_getter_returns_the_face_npc() {
        let mut e = engine();
        let this = new_npc(&mut e);
        e.set(this, TESNPC::pFaceNPC, Ptr::new(0x4242));
        assert_eq!(e.call(0x0060_44c0, &args![this]).u32(), 0x4242);
    }

    /// An NPC whose owner master file is called `file_name`, with form id
    /// `0x01000abc` and a race with form id `0x02000def`.
    fn npc_with_master(e: &mut Engine, file_name: &str) -> Ptr<TESNPC> {
        let this = new_npc(e);
        give_vtables(e, this, false, 0);
        e.set(this, TESNPC::iFormID, 0x0100_0abc);
        let race = e.mem.alloc(0x100);
        e.mem.set_u32(race + 0xc, 0x0200_0def);
        e.set(this, TESNPC::pFormRace, Ptr::new(race));
        let file = e.mem.alloc(0x100);
        e.mem.set_cstr(file + 0x20, file_name.as_bytes());
        e.register_double(GET_OWNER_MASTER, move |_, _| ret(file));
        e.mem.set_cstr(UPDATE_PREFIX, b"update");
        e.register(STRNICMP, |e, a| {
            let first = e.mem.cstr(a[0]);
            let second = e.mem.cstr(a[1]);
            let n = a[2] as usize;
            let lower = |s: &[u8]| {
                s.iter()
                    .take(n)
                    .map(|c| c.to_ascii_lowercase())
                    .collect::<Vec<_>>()
            };
            ret((lower(&first) != lower(&second)) as u32)
        });
        this
    }

    fn texture_name(
        e: &mut Engine,
        this: Ptr<TESNPC>,
        index: i32,
    ) -> (bool, Option<Vec<Vec<u32>>>) {
        e.call_log = Some(vec![]);
        let buffer = e.mem.alloc(0x104);
        let ok = e
            .call(0x0060_4310, &args![this, index, buffer, 0x104u32])
            .bool();
        let formats = calls(e, SPRINTF_S);
        (
            ok,
            if formats.is_empty() {
                None
            } else {
                Some(formats)
            },
        )
    }

    #[test]
    fn face_mod_texture_name_formats_by_race_policy_and_sex() {
        let mut e = engine();
        let this = npc_with_master(&mut e, "Dlc01.esm");
        // The component's virtual 0x2c: can be any race.
        e.mem.set_u32(FAKE_COMPONENT_VTABLE + 0x2c, 0x0ba2_0011);
        e.register(0x0ba2_0011, |_, _| ret(1));
        let (ok, formats) = texture_name(&mut e, this, 3);
        assert!(ok);
        let formats = formats.unwrap();
        assert_eq!(formats.len(), 1);
        let call = &formats[0];
        // buffer, size, format, plugin name, race id, NPC id, index.
        assert_eq!(call[1], 0x104);
        assert_eq!(call[2], FORMAT_FACE_MOD_MALE);
        assert_eq!(e.mem.cstr(call[3]), b"Dlc01.esm".to_vec());
        assert_eq!(&call[4..], &[0x00_0def, 0x00_0abc, 3]);

        e.set(this, TESNPC::iActorBaseFlags, 1);
        let (_, formats) = texture_name(&mut e, this, 0);
        assert_eq!(formats.unwrap()[0][2], FORMAT_FACE_MOD_FEMALE);

        // A fixed-race NPC: no race id in the name.
        e.register(0x0ba2_0011, |_, _| ret(0));
        let (_, formats) = texture_name(&mut e, this, 1);
        let call = &formats.unwrap()[0];
        assert_eq!(call[2], FORMAT_FACE_MOD_ANY_RACE);
        assert_eq!(&call[4..], &[0x00_0abc, 1]);
    }

    #[test]
    fn face_mod_texture_name_uses_the_master_for_update_files_and_fails_without_a_file() {
        let mut e = engine();
        let this = npc_with_master(&mut e, "Update.esm");
        let (ok, formats) = texture_name(&mut e, this, 0);
        assert!(ok);
        let call = &formats.unwrap()[0];
        assert_eq!(call[3], MASTER_FILE_NAME);
        // No owner master: false and nothing written.
        e.register(GET_OWNER_MASTER, |_, _| ret(0));
        let (ok, formats) = texture_name(&mut e, this, 0);
        assert!(!ok);
        assert!(formats.is_none());
    }

    #[test]
    fn face_mod_texture_name_follows_the_face_npc_chain_to_its_end() {
        let mut e = engine();
        let this = npc_with_master(&mut e, "Dlc01.esm");
        let middle = new_npc(&mut e);
        let last = new_npc(&mut e);
        e.set(this, TESNPC::pFaceNPC, middle.cast());
        e.set(middle, TESNPC::pFaceNPC, last.cast());
        e.set(last, TESNPC::iFormID, 0x0100_0777);
        let race = e.mem.alloc(0x100);
        e.set(last, TESNPC::pFormRace, Ptr::new(race));
        give_vtables(&mut e, last, false, 1);
        let (ok, formats) = texture_name(&mut e, this, 2);
        assert!(ok);
        // The last NPC's id (a fixed-race NPC: id and index only).
        assert_eq!(&formats.unwrap()[0][4..], &[0x00_0777, 2]);
    }

    #[test]
    fn face_texture_slot_is_cleared_defaulted_or_loaded() {
        let mut e = engine();
        let this = npc_with_master(&mut e, "Dlc01.esm");
        let slot = e.mem.alloc(4);
        let tes = e.mem.alloc(0x10);
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER_SINGLETON, player);
        e.set_global(TES_SINGLETON, tes);
        e.register(0x0045_68c0, |e, a| {
            // The loader writes the texture it made into the NiPointer.
            e.mem.set_u32(a[2], 0x7e57);
            ret(0)
        });
        e.mem.set_u32(slot, 0xdead);
        // The setting at 011d5adc set: cleared.
        e.mem.set_u8(0x011d_5adc + 4, 1);
        e.call(0x0060_44e0, &args![this, 0u32, slot]);
        assert_eq!(e.mem.u32(slot), 0);
        // The setting clear: loaded from the file name built.
        e.mem.set_u8(0x011d_5adc + 4, 0);
        e.call(0x0060_44e0, &args![this, 0u32, slot]);
        assert_eq!(e.mem.u32(slot), 0x7e57);
        // A texture index above 0 uses the default modulation texture.
        let manager = e.mem.alloc(0x1200);
        e.mem.set_u32(manager + 0x119c, 0xd0d0);
        e.set_global(FACE_GEN_MANAGER, manager);
        e.call(0x0060_44e0, &args![this, 1u32, slot]);
        assert_eq!(e.mem.u32(slot), 0xd0d0);
        // The player's base form never gets one.
        e.mem.set_u32(player + 0x20, this.addr());
        e.call(0x0060_44e0, &args![this, 0u32, slot]);
        assert_eq!(e.mem.u32(slot), 0);
        // A failing name (no master file): cleared.
        e.mem.set_u32(player + 0x20, 0);
        e.mem.set_u32(slot, 0xdead);
        e.register(GET_OWNER_MASTER, |_, _| ret(0));
        e.call(0x0060_44e0, &args![this, 0u32, slot]);
        assert_eq!(e.mem.u32(slot), 0);
    }

    #[test]
    fn default_modulation_texture_is_read_from_the_manager_or_is_null() {
        let mut e = engine();
        assert_eq!(e.call(0x0060_45f0, &args![]).u32(), 0);
        let manager = e.mem.alloc(0x1200);
        e.mem.set_u32(manager + 0x119c, 0xd0d0);
        e.set_global(FACE_GEN_MANAGER, manager);
        assert_eq!(e.call(0x0060_45f0, &args![]).u32(), 0xd0d0);
    }

    #[test]
    fn voice_type_defaults_for_the_player_and_otherwise_comes_from_stored_or_race() {
        let mut e = engine();
        let this = new_npc(&mut e);
        let component = this.addr() + COMPONENT_ACTOR_BASE_DATA;
        // 58db10(index) is the default object lookup.
        e.register(0x0058_db10, |_, a| ret(0xd000 + a[0]));
        let player = e.mem.alloc(0x800);
        e.set_global(PLAYER_SINGLETON, player);
        // A stored voice type wins.
        e.mem.set_u32(component + 0x20, 0xaaaa);
        assert_eq!(e.call(0x0060_46a0, &args![component]).u32(), 0xaaaa);
        // Without one, the race's voice type for the sex; no race: null.
        e.mem.set_u32(component + 0x20, 0);
        assert_eq!(e.call(0x0060_46a0, &args![component]).u32(), 0);
        let race = e.mem.alloc(0x600);
        e.mem.set_u32(race + 0x4fc, 0xb0);
        e.mem.set_u32(race + 0x500, 0xb1);
        e.set(this, TESNPC::pFormRace, Ptr::new(race));
        assert_eq!(e.call(0x0060_46a0, &args![component]).u32(), 0xb0);
        e.set(this, TESNPC::iActorBaseFlags, 1);
        assert_eq!(e.call(0x0060_46a0, &args![component]).u32(), 0xb1);
        // The player's base form: default objects by sex and the byte at
        // +0x7c5 of the player.
        e.mem.set_u32(player + 0x20, this.addr());
        e.mem.set_u32(component + 0x20, 0);
        e.set(this, TESNPC::iActorBaseFlags, 0);
        assert_eq!(e.call(0x0060_46a0, &args![component]).u32(), 0xd00e);
        e.mem.set_u32(component + 0x20, 0);
        e.mem.set_u8(player + 0x7c5, 1);
        assert_eq!(e.call(0x0060_46a0, &args![component]).u32(), 0xd00f);
        e.mem.set_u32(component + 0x20, 0);
        e.set(this, TESNPC::iActorBaseFlags, 1);
        assert_eq!(e.call(0x0060_46a0, &args![component]).u32(), 0xd011);
        e.mem.set_u8(player + 0x7c5, 0);
        e.mem.set_u32(component + 0x20, 0);
        assert_eq!(e.call(0x0060_46a0, &args![component]).u32(), 0xd010);
    }

    #[test]
    fn race_voice_and_player_flag_getters() {
        let mut e = engine();
        let race = e.mem.alloc(0x600);
        e.mem.set_u32(race + 0x4fc + 8, 0xc0de);
        assert_eq!(e.call(0x0060_4780, &args![race, 2u32]).u32(), 0xc0de);
        let player = e.mem.alloc(0x800);
        e.mem.set_u8(player + 0x7c5, 1);
        assert_eq!(e.call(0x0060_47a0, &args![player]).u8(), 1);
    }

    /// An auto-calc NPC with a race and a class, and the doubles of the
    /// attribute and skill formulas: the class's seven attribute bytes are
    /// `[5, 12, 10, 0, 3, 3, 9]`, every skill input and the luck-like byte
    /// is 3, the derived skill value is `20 + stat + luck`, the level is 5
    /// and the budget per level 10 (so each tag skill receives
    /// `2 * floor(40 / 3) = 26`), the float setting added to a tag skill is
    /// 0.5, and skills `0x20` and `0x21` are the class's tag skills.
    fn values_world() -> (Engine, Ptr<TESNPC>, u32) {
        let mut e = engine();
        let this = new_npc(&mut e);
        let base = this.addr();
        give_vtables(&mut e, this, true, 0);
        let race = e.mem.alloc(0x100);
        e.set(this, TESNPC::pFormRace, Ptr::new(race));
        let class = e.mem.alloc(0x100);
        e.mem.set_u32(class + 0xc, 0x1234);
        e.set(this, TESNPC::pCl, Ptr::new(class));
        e.mem.set_u16(base + COMPONENT_ACTOR_BASE_DATA + 8, 5);
        for (i, v) in [5u8, 12, 10, 0, 3, 3, 9].iter().enumerate() {
            e.mem.set_u8(class + 0x38 + 4 + i as u32, *v);
        }
        for j in 0..14 {
            e.mem.set_u8(base + 0xb7 + j, 3);
        }
        e.mem.set_u8(base + COMPONENT_ATTRIBUTES + 10, 3);
        e.register(TO_ACTOR_VALUE, |_, a| {
            ret(match a[0] {
                0 => a[1],
                2 => 0x20 + a[1],
                _ => 0,
            })
        });
        e.register(ATTRIBUTES_GET, |e, a| ret(e.mem.u8(a[0] + a[1] - 1) as u32));
        e.register(ROUND_FLOAT, |_, a| {
            ret(f32::from_bits(a[0]).round_ties_even() as i32 as u32)
        });
        e.register(0x0064_8c10, |_, _| ret(10));
        e.register(0x0040_4040, |_, a| ret_float(f32::from_bits(a[0]).floor()));
        e.register(0x0047_f060, |_, a| ret((0x20..0x2e).contains(&a[0]) as u32));
        e.register(0x0064_3bf0, |e, a| {
            e.mem.set_u32(a[1], a[0] - 0x20);
            e.mem.set_f32(a[2], 20.0);
            ret(0)
        });
        e.register(0x0064_3c20, |_, a| {
            ret_float(f32::from_bits(a[0]) + f32::from_bits(a[1]) + f32::from_bits(a[2]))
        });
        e.register(0x005a_5f40, |_, a| {
            ret((a[1] == 0x20 || a[1] == 0x21) as u32)
        });
        e.register(0x0062_86d0, |_, _| ret(0x321));
        e.mem.set_u32(0x011c_d6c0 + 4, 2);
        e.mem.set_f32(0x011c_cf5c + 4, 0.5);
        e.set_global(0x0102_0758, 10.0f64);
        e.set_global(0x0101_7b78, 10.0f32);
        e.set_global(0x0101_7a40, 100.0f64);
        e.set_global(0x0101_6410, 100.0f32);
        (e, this, class)
    }

    fn skills(e: &Engine, this: Ptr<TESNPC>) -> Vec<u8> {
        (0..14).map(|j| e.mem.u8(this.addr() + 0x114 + j)).collect()
    }

    #[test]
    fn init_values_computes_attributes_and_skills_from_the_class() {
        let (mut e, this, _) = values_world();
        let base = this.addr();
        started(&mut e);
        e.call(0x0060_3be0, &args![this, 0u32]);
        // The seven attributes: the class's bytes, at most 10.
        let sets: Vec<Vec<u32>> = calls(&mut e, ATTRIBUTES_SET);
        let attributes = base + COMPONENT_ATTRIBUTES;
        let expected = [5u32, 10, 10, 0, 3, 3, 9];
        assert_eq!(sets.len(), 7);
        for (i, set) in sets.iter().enumerate() {
            assert_eq!(set, &vec![attributes, i as u32, expected[i], 0]);
        }
        // Tag skills get the class share and the setting: 26 + 0.5 + 26.
        let mut want = vec![26u8; 14];
        want[0] = 52;
        want[1] = 52;
        assert_eq!(skills(&e, this), want);
        // The AI value is only touched for NPCs whose component says so, and
        // the form is marked changed (virtual 0x48, 0x20c).
        assert!(calls(&mut e, 0x0047_efb0).is_empty());
        assert!(calls(&mut e, FAKE_NOOP).contains(&vec![base, 0x20c]));
    }

    #[test]
    fn init_values_hands_a_tag_skills_overflow_to_the_following_ones() {
        let (mut e, this, _) = values_world();
        e.mem.set_f32(0x011c_cf5c + 4, 100.0);
        e.call(0x0060_3be0, &args![this, 0u32]);
        // 100 + 26 + 26 = 152: the skill saturates at 100 and 26 (52 / 2) is
        // carried to the next tag skill: 100 + 52 + 26 = 178.
        let mut want = vec![26u8; 14];
        want[0] = 100;
        want[1] = 100;
        assert_eq!(skills(&e, this), want);
        // With the extra divisor the budget is split in four: floor(40 / 4)
        // is 10, so the tag skills get 20.
        let (mut e, this, _) = values_world();
        e.call(0x0060_3be0, &args![this, 1u32]);
        let mut want = vec![26u8; 14];
        want[0] = 46; // 0.5 + 20 + 26 = 46.5, rounded to even
        want[1] = 46;
        assert_eq!(skills(&e, this), want);
    }

    #[test]
    fn init_values_sets_the_ai_value_from_the_class_when_the_component_asks() {
        let (mut e, this, _) = values_world();
        let base = this.addr();
        e.mem.set_u32(FAKE_COMPONENT_VTABLE + 0x30, 0x0ba2_0020);
        e.register(0x0ba2_0020, |_, _| ret(1));
        started(&mut e);
        e.call(0x0060_3be0, &args![this, 0u32]);
        assert_eq!(
            calls(&mut e, 0x0047_efb0),
            vec![vec![base + COMPONENT_AI_FORM, 0x321, 0]]
        );
        // Without a class the value is zero.
        let (mut e, this, _) = values_world();
        e.set(this, TESNPC::pCl, Ptr::NULL);
        started(&mut e);
        e.call(0x0060_3be0, &args![this, 0u32]);
        // (Without a class nothing is computed at all.)
        assert!(calls(&mut e, ATTRIBUTES_SET).is_empty());
    }

    #[test]
    fn init_values_ignores_the_class_for_the_player_base_with_that_class() {
        let (mut e, this, _) = values_world();
        e.set(this, TESNPC::iFormID, PLAYER_BASE_FORM_ID);
        // The setting at 011d0e7c holds the class's form id.
        e.mem.set_u32(0x011d_0e7c + 4, 0x1234);
        started(&mut e);
        e.call(0x0060_3be0, &args![this, 0u32]);
        // Attributes are all zero and no tag bonus is added: 20 + 3 + 3.
        assert!(calls(&mut e, ATTRIBUTES_SET).iter().all(|set| set[2] == 0));
        assert_eq!(skills(&e, this), vec![26u8; 14]);
        // A different class form id: the class is used again.
        let (mut e, this, _) = values_world();
        e.set(this, TESNPC::iFormID, PLAYER_BASE_FORM_ID);
        e.mem.set_u32(0x011d_0e7c + 4, 0x9999);
        e.call(0x0060_3be0, &args![this, 0u32]);
        assert_eq!(skills(&e, this)[0], 52);
    }

    #[test]
    fn init_values_does_nothing_without_race_class_or_auto_calc() {
        let (mut e, this, _) = values_world();
        e.set(this, TESNPC::pFormRace, Ptr::NULL);
        started(&mut e);
        e.call(0x0060_3be0, &args![this, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        let (mut e, this, _) = values_world();
        give_vtables(&mut e, this, false, 2);
        started(&mut e);
        e.call(0x0060_3be0, &args![this, 0u32]);
        assert!(calls(&mut e, ATTRIBUTES_SET).is_empty());
        assert_eq!(skills(&e, this), vec![0u8; 14]);
    }

    /// An actor with the process, vtable slots and inventory the worn-object
    /// code uses. `items` maps a slot to the form of the inventory item
    /// there. Returns the actor, the process and the free-list of items.
    fn worn_world(items: &[(u32, u32)]) -> (Engine, Ptr<TESNPC>, u32, u32) {
        let mut e = engine();
        let this = new_npc(&mut e);
        let actor = e.mem.alloc(0x100);
        let process = e.mem.alloc(0x100);
        e.mem.set_u32(actor + 0x68, process);
        put_object_vtable(
            &mut e,
            actor,
            0x0bc0_0000,
            &[
                (0x100, 0x0bc1_0000),
                (0x184, 0x0bc1_0001),
                (0x1d0, 0x0bc1_0002),
                (0x22c, 0x0bc1_0003),
            ],
        );
        e.register(0x0bc1_0000, |_, _| ret(1));
        e.register(0x0bc1_0001, |_, _| ret(0));
        e.register(0x0bc1_0002, |_, _| ret(0x5151));
        e.register(0x0bc1_0003, |_, _| ret(0));
        put_object_vtable(
            &mut e,
            process,
            0x0bc2_0000,
            &[
                (0x160, 0x0bc3_0000),
                (0x164, 0x0bc3_0001),
                (0x168, 0x0bc3_0002),
                (0x16c, 0x0bc3_0003),
            ],
        );
        for i in 0..4 {
            e.register(0x0bc3_0000 + i, |_, _| ret(0));
        }
        let inventory = e.mem.alloc(0x40);
        e.register_double(0x004b_f220, move |_, _| ret(inventory));
        let mut table = vec![];
        for (slot, form) in items {
            let item = e.mem.alloc(0x20);
            e.mem.set_u32(item + 8, *form);
            table.push((*slot, item));
        }
        let table2 = table.clone();
        e.register_double(0x004c_8220, move |_, a| {
            ret(table2
                .iter()
                .find(|(slot, _)| *slot == a[2])
                .map_or(0, |(_, item)| *item))
        });
        e.set_global(0x011d_df38, 0x7000_0000u32);
        let first_item = table.first().map_or(0, |(_, item)| *item);
        (e, this, actor, first_item)
    }

    #[test]
    fn worn_objects_are_equipped_slot_by_slot_around_the_flag_object() {
        let (mut e, this, actor, first_item) = worn_world(&[(0, 0xf0), (5, 0xf5)]);
        // The first item has extra data: a word pointing at a block whose
        // first word is the extra.
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, 0xe0);
        e.mem.set_u32(first_item, block);
        e.register_double(0x004623f0, |_, a| ret(if a[1] == 0 { 1 } else { 0 }));
        started(&mut e);
        e.call(0x0060_47c0, &args![this, actor, 0u32, false, 0u32, false]);
        // The flag object is switched off, and restored with its old value.
        let flag = calls(&mut e, 0x004623f0);
        assert_eq!(flag, vec![vec![0x7000_0000, 0], vec![0x7000_0000, 1]]);
        // The actor was asked to drop its weapon and worn objects.
        assert_eq!(calls(&mut e, 0x0057_1b50), vec![vec![actor]]);
        assert_eq!(calls(&mut e, 0x004b_fe50).len(), 1);
        // The process was reset by four virtual calls.
        assert_eq!(
            calls(&mut e, 0x0bc3_0002),
            vec![vec![e.mem.u32(actor + 0x68), 0]]
        );
        assert_eq!(calls(&mut e, 0x0bc3_0000).len(), 1);
        // Two equips: form, 1, extra, 0.
        assert_eq!(
            calls(&mut e, 0x0bc1_0001),
            vec![vec![actor, 0xf0, 1, 0xe0, 0], vec![actor, 0xf5, 1, 0, 0]]
        );
        // Each item is released with flag 1.
        assert_eq!(calls(&mut e, 0x0044_59e0).len(), 2);
        // Permanent magic is cast at the end.
        assert_eq!(calls(&mut e, 0x008c_26e0), vec![vec![actor, 1]]);
    }

    #[test]
    fn worn_objects_skip_slots_covered_by_the_body_slot_model() {
        let (mut e, this, actor, _) = worn_world(&[(2, 0xf2), (7, 0xf7), (9, 0xf9)]);
        // Slot 2 is the body: its form (cast to a biped model, which the
        // double leaves unchanged) is remembered, and then asked whether it
        // fills slot 7.
        e.register(FILLS_BIPED_SLOT, |_, a| ret((a[1] == 7) as u32));
        // GetFormAsBipedModel for the other slots: a model that does not
        // cover the body.
        e.register(0x0048_0db0, |_, a| ret(a[0]));
        started(&mut e);
        e.call(0x0060_47c0, &args![this, actor, 0u32, false, 0u32, true]);
        let equips = calls(&mut e, 0x0bc1_0001);
        let forms: Vec<u32> = equips.iter().map(|call| call[1]).collect();
        assert_eq!(forms, vec![0xf2, 0xf9]);
        // With the flag kept, the flag object is not touched.
        assert!(calls(&mut e, 0x004623f0).is_empty());
    }

    #[test]
    fn worn_objects_do_not_equip_an_item_that_fills_the_body_slot() {
        let (mut e, this, actor, _) = worn_world(&[(4, 0xf4)]);
        e.register(0x0048_0db0, |_, a| ret(a[0]));
        e.register(FILLS_BIPED_SLOT, |_, a| ret((a[1] == 2) as u32));
        started(&mut e);
        e.call(0x0060_47c0, &args![this, actor, 0u32, false, 0u32, true]);
        assert!(calls(&mut e, 0x0bc1_0001).is_empty());
        // It is still released.
        assert_eq!(calls(&mut e, 0x0044_59e0).len(), 1);
    }

    #[test]
    fn worn_weapon_is_equipped_or_handed_to_the_process() {
        // Weapon found, not carried: equipped with Actor::EquipObject.
        let (mut e, this, actor, _) = worn_world(&[]);
        let weapon = e.mem.alloc(0x20);
        e.mem.set_u32(weapon + 8, 0xc1);
        e.mem.set_u32(weapon + 4, 2);
        e.register_double(0x004c_7400, move |_, _| ret(weapon));
        started(&mut e);
        e.call(0x0060_47c0, &args![this, actor, 0u32, true, 0u32, true]);
        assert_eq!(
            calls(&mut e, 0x0088_c830),
            vec![vec![actor, 0xc1, 2, 0, 1, 0, 1]]
        );
        // Without extra data the weapon entry is released.
        assert_eq!(calls(&mut e, 0x0044_59e0), vec![vec![weapon, 1]]);

        // Already carried (the actor has it): it goes to the process unless
        // it is the current weapon.
        let (mut e, this, actor, _) = worn_world(&[]);
        let weapon = e.mem.alloc(0x20);
        e.mem.set_u32(weapon + 8, 0xc1);
        e.register_double(0x004c_7400, move |_, _| ret(weapon));
        e.register(0x0057_5400, |_, _| ret(1));
        e.register(0x008a_1710, |_, _| ret(0xc2));
        started(&mut e);
        e.call(0x0060_47c0, &args![this, actor, 0u32, true, 0u32, true]);
        let process = e.mem.u32(actor + 0x68);
        let handed = calls(&mut e, 0x0bc3_0000);
        // The reset call, then the weapon (with the actor's virtual 0x1d0
        // result).
        assert_eq!(handed.len(), 2);
        assert_eq!(handed[1], vec![process, weapon, 0x5151, 0]);
        // The current weapon is left alone.
        e.register(0x008a_1710, |_, _| ret(0xc1));
        e.call_log = Some(vec![]);
        e.call(0x0060_47c0, &args![this, actor, 0u32, true, 0u32, true]);
        assert_eq!(calls(&mut e, 0x0bc3_0000).len(), 1);
        // An actor that refuses (virtual 0x22c) gets no weapon.
        e.register(0x0bc1_0003, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0060_47c0, &args![this, actor, 0u32, true, 0u32, true]);
        assert!(calls(&mut e, 0x004c_7400).is_empty());
    }

    /// A template world: an NPC `npc` whose actor-base component (`npc +
    /// 0x30`) has the template NPC `template`, with every template-use flag
    /// in `mask` set, and recognisable values in the template.
    fn template_world(mask: u32) -> (Engine, Ptr<TESNPC>, Ptr<TESNPC>) {
        let mut e = engine();
        let npc = new_npc(&mut e);
        let template = new_npc(&mut e);
        give_vtables(&mut e, npc, false, 0);
        give_vtables(&mut e, template, false, 1);
        // The component's template pointer (+0x24).
        e.mem.set_u32(npc.addr() + 0x54, template.addr());
        e.register_double(USES_TEMPLATE_FLAG, move |_, a| ret((mask >> a[1]) & 1));
        e.register(0x0071_7e50, |_, a| ret(a[0] + 4));
        e.register(0x0044_1110, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        e.register(0x0047_d370, |_, _| ret(25));
        e.register(0x0047_d390, |_, _| ret(10));
        e.register(0x0047_d3b0, |_, _| ret(30));
        e.register(0x008f_21d0, |_, _| ret(100));
        e.register(0x0046_1560, |_, _| ret(0));
        e.register(0x0047_d1a0, |e, a| {
            e.mem.set_u16(a[1], 3);
            ret(0x7777)
        });
        e.set(template, TESNPC::iFormID, PLAYER_BASE_FORM_ID);
        e.set(template, TESNPC::fWeight, 80.0);
        e.set(template, TESNPC::fHeight, 1.7);
        e.set(template, TESNPC::pCl, Ptr::new(0xc1a5));
        e.set(template, TESNPC::pCombatStyle, Ptr::new(0x88));
        e.set(template, TESNPC::pFormRace, Ptr::new(0x6ace));
        e.set(template, TESNPC::iActorBaseFlags, 1);
        e.mem
            .set_u32(template.addr() + COMPONENT_ACTOR_BASE_DATA + 0x1c, 0x5eed);
        for i in 0..0xe {
            e.mem.set_u8(template.addr() + 0x114 + i, 10 + i as u8);
            e.mem.set_u8(template.addr() + 0x122 + i, 40 + i as u8);
        }
        (e, npc, template)
    }

    #[test]
    fn template_copy_does_nothing_without_a_template() {
        let (mut e, npc, _) = template_world(0x3ff);
        e.mem.set_u32(npc.addr() + 0x54, 0);
        started(&mut e);
        let component = npc.addr() + COMPONENT_ACTOR_BASE_DATA;
        e.call(0x0060_4ba0, &args![component, Ptr::<()>::NULL]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn template_copy_takes_each_flagged_part_from_the_template() {
        let (mut e, npc, template) = template_world(0x3ff);
        let c = npc.addr() + COMPONENT_ACTOR_BASE_DATA;
        let t = template.addr();
        // The NPC keeps part of its flags, the template adds its part.
        e.set(npc, TESNPC::iActorBaseFlags, 0x0000_0001);
        e.set(template, TESNPC::iActorBaseFlags, 0x0000_0003);
        started(&mut e);
        e.call(0x0060_4ba0, &args![c, Ptr::<()>::NULL]);
        let at = |e: &mut Engine, addr: u32| calls(e, addr);
        assert_eq!(at(&mut e, 0x0048_cce0), vec![vec![c + 0xc4, t + 0xf4]]);
        assert_eq!(at(&mut e, 0x0048_1c80), vec![vec![c + 0x34, t + 0x64]]);
        assert_eq!(at(&mut e, 0x0048_70f0), vec![vec![c + 0xa0, t + 0xd0]]);
        // Own flags in 0xef9f5df5 (bit 0 is), the template's in 0x1060a20a
        // (bit 1 is): 1 | 2.
        assert_eq!(at(&mut e, 0x0047_dd30), vec![vec![c, 3]]);
        assert_eq!(at(&mut e, 0x0047_fb50), vec![vec![c + 0x94, t + 0xc4]]);
        assert_eq!(at(&mut e, 0x0050_eaa0), vec![vec![npc.addr(), t]]);
        assert_eq!(at(&mut e, 0x0047_86e0), vec![vec![c + 0xd4, t + 0x104]]);
        // Voice, race, height, weight, class, combat style.
        assert_eq!(at(&mut e, 0x0050_f9a0), vec![vec![c, 0x5eed]]);
        assert_eq!(at(&mut e, 0x0050_f9c0), vec![vec![c, 0]]);
        assert_eq!(at(&mut e, 0x0047_dd50), vec![vec![c, 1, 1, 1]]);
        assert_eq!(e.get(npc, TESNPC::pFormRace), Ptr::new(0x6ace));
        assert_eq!(
            at(&mut e, 0x0094_42e0),
            vec![vec![npc.addr(), 1.7f32.to_bits()]]
        );
        assert_eq!(e.get(npc, TESNPC::fWeight), 80.0);
        assert_eq!(at(&mut e, NPC_SET_CLASS), vec![vec![npc.addr(), 0xc1a5]]);
        assert_eq!(e.get(npc, TESNPC::pCombatStyle), Ptr::new(0x88));
        // Level, flags, bounds, attributes, health, speed, fatigue and the
        // two byte arrays.
        assert_eq!(at(&mut e, 0x0047_dfe0), vec![vec![c, 25]]);
        assert_eq!(at(&mut e, 0x0047_de40), vec![vec![c, 10]]);
        assert_eq!(at(&mut e, 0x0047_de70), vec![vec![c, 30]]);
        assert_eq!(at(&mut e, 0x0048_0000), vec![vec![c + 0x88, t + 0xb8]]);
        assert_eq!(at(&mut e, 0x0048_7240), vec![vec![c + 0x80, t + 0xb0]]);
        assert_eq!(at(&mut e, 0x0047_e270), vec![vec![c, 100]]);
        assert_eq!(e.mem.bytes(c + 0xe4, 0xe), e.mem.bytes(t + 0x114, 0xe));
        assert_eq!(e.mem.bytes(c + 0xf2, 0xe), e.mem.bytes(t + 0x122, 0xe));
        // Factions, spells, touch spell, AI data and packages.
        assert_eq!(at(&mut e, 0x0047_ca20), vec![vec![c, t + 0x30]]);
        assert_eq!(at(&mut e, 0x0048_d630), vec![vec![c + 0x4c, t + 0x7c]]);
        assert_eq!(at(&mut e, 0x0047_bdd0), vec![vec![c + 0x40, t + 0x70]]);
        assert_eq!(at(&mut e, 0x0047_f1e0), vec![vec![c + 0x60, t + 0x90]]);
        assert_eq!(at(&mut e, 0x0047_f2d0), vec![vec![c + 0x60, t + 0x90]]);
        // The template flags are rewritten and the form marked.
        assert_eq!(at(&mut e, 0x0070_37c0), vec![vec![c, 0x7777]]);
        assert_eq!(at(&mut e, 0x0047_cd90), vec![vec![c, 3]]);
        assert_eq!(at(&mut e, 0x0047_ccc0), vec![vec![c, 1]]);
        // The temporary NPC is destroyed.
        assert_eq!(at(&mut e, TESNPC_DESTRUCT).len(), 1);
    }

    #[test]
    fn template_copy_copies_only_the_flagged_parts() {
        // Only bit 2 (factions).
        let (mut e, npc, template) = template_world(1 << 2);
        let c = npc.addr() + COMPONENT_ACTOR_BASE_DATA;
        started(&mut e);
        e.call(0x0060_4ba0, &args![c, Ptr::<()>::NULL]);
        assert_eq!(
            calls(&mut e, 0x0047_ca20),
            vec![vec![c, template.addr() + 0x30]]
        );
        for untouched in [
            0x0048_cce0,
            0x0048_1c80,
            0x0048_70f0,
            0x0047_dd30,
            0x0047_dfe0,
        ] {
            assert!(calls(&mut e, untouched).is_empty());
        }
    }

    #[test]
    fn template_copy_prefers_an_explicit_npc_over_the_stored_template() {
        let (mut e, npc, _) = template_world(1 << 2);
        let other = new_npc(&mut e);
        give_vtables(&mut e, other, false, 2);
        let c = npc.addr() + COMPONENT_ACTOR_BASE_DATA;
        started(&mut e);
        e.call(0x0060_4ba0, &args![c, other]);
        assert_eq!(
            calls(&mut e, 0x0047_ca20),
            vec![vec![c, other.addr() + 0x30]]
        );
    }

    #[test]
    fn template_copy_builds_a_default_source_for_a_leveled_template() {
        let (mut e, npc, _) = template_world(1 << 0);
        // The stored template is a leveled form (047cdd0): no source, so a
        // default NPC with the data handler's first race and class.
        e.register(0x0047_cdd0, |_, _| ret(1));
        let handler = e.mem.alloc(0x100);
        e.register(DATA_HANDLER_FIRST_LIST, |_, a| ret(a[0] + 0x60));
        e.register(DATA_HANDLER_SECOND_LIST, |_, a| ret(a[0] + 0x80));
        // The default NPC the code builds on the stack carries the real
        // vtables: give them the slots the copy calls.
        e.mem.set_u32(0x0104_a2f4 + 0x188, FAKE_GET_COMBAT_STYLE);
        e.mem.set_u32(0x0104_a284 + 0x68, FAKE_NOOP);
        e.mem.set_u32(0x0104_a284 + 0x64, FAKE_NOOP);
        e.set_global(DATA_HANDLER, handler);
        e.mem.set_u32(handler + 0x60, 0x6ace_0001);
        e.mem.set_u32(handler + 0x80, 0xc1a5_0002);
        let c = npc.addr() + COMPONENT_ACTOR_BASE_DATA;
        started(&mut e);
        e.call(0x0060_4ba0, &args![c, Ptr::<()>::NULL]);
        // (The default NPC's constructor also sets health through the same
        // setter; the race is the call that carries the handler's word.)
        let race = calls(&mut e, SET_WORD_AT_4);
        let stored = race.iter().find(|c| c[1] == 0x6ace_0001).unwrap();
        let default_npc = stored[0] - COMPONENT_RACE;
        assert_eq!(
            calls(&mut e, NPC_SET_CLASS)[0],
            vec![default_npc, 0xc1a5_0002]
        );
        assert_eq!(e.get(npc, TESNPC::pFormRace), Ptr::new(0x6ace_0001));
    }

    #[test]
    fn weight_setter_and_script_forwarder() {
        let mut e = engine();
        let this = new_npc(&mut e);
        e.register(0x0050_fbf0, |_, a| ret(a[0] + a[1]));
        assert_eq!(
            e.call(0x0060_55d0, &args![this, 5u32]).u32(),
            this.addr() + 5
        );
    }

    /// A reference whose virtual `0x1e8` returns the biped animation stored
    /// at `+0x100` and whose `0x1f0` stores one there.
    fn reference_with_biped(e: &mut Engine, biped: u32) -> u32 {
        let reference = e.mem.alloc(0x200);
        put_object_vtable(
            e,
            reference,
            0x0bd0_0000,
            &[
                (0x1e8, 0x0bd1_0000),
                (0x1f0, 0x0bd1_0001),
                (0x178, 0x0bd1_0002),
            ],
        );
        e.register(0x0bd1_0000, |e, a| ret(e.mem.u32(a[0] + 0x100)));
        e.register(0x0bd1_0001, |e, a| {
            e.mem.set_u32(a[0] + 0x100, a[1]);
            ret(0)
        });
        e.mem.set_u32(reference + 0x100, biped);
        reference
    }

    #[test]
    fn create_biped_anim_makes_one_or_reports_an_existing_one() {
        let mut e = engine();
        let this = new_npc(&mut e);
        give_vtables(&mut e, this, false, 0);
        let reference = reference_with_biped(&mut e, 0);
        e.register(0x004a_aca0, |_, a| ret(a[0]));
        started(&mut e);
        let created = e.call(0x0060_55f0, &args![this, reference]).u32();
        assert_ne!(created, 0);
        assert_eq!(e.mem.u32(reference + 0x100), created);
        let guard = calls(&mut e, 0x0040_4eb0);
        assert_eq!(guard.len(), 1);
        assert_eq!(&guard[0][1..], &[0x33, 1, 0x0104_a77c, 0x15d1]);
        assert_eq!(calls(&mut e, 0x0040_4ee0), vec![vec![guard[0][0]]]);
        assert_eq!(
            calls(&mut e, 0x004a_aca0),
            vec![vec![created, reference, 0]]
        );
        // The second time the existing one is returned and reported.
        e.call_log = Some(vec![]);
        let again = e.call(0x0060_55f0, &args![this, reference]).u32();
        assert_eq!(again, created);
        assert_eq!(
            calls(&mut e, 0x005b_5e40),
            vec![vec![0x0104_a720, 0x0777_0000]]
        );
        assert!(calls(&mut e, 0x004a_aca0).is_empty());
    }

    #[test]
    fn clone_3d_delegates_to_the_base_form_for_a_templated_npc() {
        let mut e = engine();
        let this = new_npc(&mut e);
        let reference = reference_with_biped(&mut e, 0);
        let base_form = reference_with_biped(&mut e, 0);
        e.mem.set_u32(reference + 0x20, base_form);
        // The base form is another object whose virtual 0x178 answers.
        put_object_vtable(&mut e, base_form, 0x0bd2_0000, &[(0x178, 0x0bd2_0001)]);
        e.register(0x0bd2_0001, |_, _| ret(0x1234));
        e.register(0x0047_cdb0, |_, _| ret(1));
        started(&mut e);
        let result = e.call(0x0060_5410, &args![this, reference]).u32();
        assert_eq!(result, 0x1234);
        assert_eq!(
            calls(&mut e, 0x0047_ce10),
            vec![vec![this.addr() + COMPONENT_ACTOR_BASE_DATA, reference]]
        );
        assert_eq!(calls(&mut e, 0x0bd2_0001), vec![vec![base_form, reference]]);
        // A reference that 0056afc0 excludes is cloned normally.
        e.register(0x0056_afc0, |_, _| ret(1));
        e.register(0x0050_ee50, |_, _| ret(0x4d4d));
        let result = e.call(0x0060_5410, &args![this, reference]).u32();
        assert_eq!(result, 0x4d4d);
    }

    #[test]
    fn clone_3d_attaches_a_biped_animation_to_the_model() {
        let mut e = engine();
        let this = new_npc(&mut e);
        e.register(0x0050_ee50, |_, _| ret(0x4d4d));
        e.register(FIND_NODE_BY_NAME, |_, a| ret(a[0]));
        e.register(0x004a_aca0, |_, a| ret(a[0]));
        // An existing biped animation is rebuilt on the model.
        let reference = reference_with_biped(&mut e, 0xb100);
        started(&mut e);
        let model = e.call(0x0060_5410, &args![this, reference]).u32();
        assert_eq!(model, 0x4d4d);
        assert_eq!(
            calls(&mut e, 0x0050_ee50),
            vec![vec![this.addr(), reference, 0]]
        );
        assert_eq!(
            calls(&mut e, FIND_NODE_BY_NAME),
            vec![vec![0x4d4d, ROOT_NODE_NAME]]
        );
        assert_eq!(calls(&mut e, 0x004a_ad00), vec![vec![0xb100, 0x4d4d]]);
        assert_eq!(calls(&mut e, 0x00a5_a040), vec![vec![0x4d4d]]);
        let update = calls(&mut e, 0x00a5_9c60);
        let data = calls(&mut e, 0x0043_d410);
        assert_eq!(data.len(), 1);
        assert_eq!(&data[0][1..], &[0.0f32.to_bits(), 0, 0]);
        assert_eq!(update, vec![vec![0x4d4d, data[0][0]]]);

        // Without one, a new one is made and given to the reference.
        let reference = reference_with_biped(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0060_5410, &args![this, reference]);
        let made = e.mem.u32(reference + 0x100);
        assert_ne!(made, 0);
        assert_eq!(
            calls(&mut e, 0x004a_aca0),
            vec![vec![made, reference, 0x4d4d]]
        );
        assert!(calls(&mut e, 0x004a_ad00).is_empty());

        // A model without the root node is left alone; a null reference
        // gives no model.
        e.register(FIND_NODE_BY_NAME, |_, _| ret(0));
        let reference = reference_with_biped(&mut e, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060_5410, &args![this, reference]).u32(), 0x4d4d);
        assert_eq!(e.mem.u32(reference + 0x100), 0);
        assert!(calls(&mut e, 0x00a5_a040).is_empty());
        assert_eq!(e.call(0x0060_5410, &args![this, 0u32]).u32(), 0);
    }

    #[test]
    fn the_small_settings_and_the_slot_name_table() {
        let mut e = engine();
        e.mem.set_u8(0x011d_5a30 + 4, 1);
        assert_eq!(e.call(0x0060_5d20, &args![]).u8(), 1);
        e.mem.set_u8(0x011d_5a30 + 4, 0);
        assert_eq!(e.call(0x0060_5d20, &args![]).u8(), 0);
        e.mem.set_u8(0x011c_b99c + 4, 1);
        assert_eq!(e.call(0x0060_5d50, &args![]).u8(), 1);
        e.mem.set_u32(0x0119_9fc4 + 8, 0xcafe);
        assert_eq!(e.call(0x0060_5d40, &args![2u32]).u32(), 0xcafe);
    }

    type Applied = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    /// What `fn_006056f0` handed to the face-gen parameter block.
    struct HeadWorld {
        e: Engine,
        npc: Ptr<TESNPC>,
        actor: u32,
        head: u32,
        applied: Applied,
        words: Rc<RefCell<Vec<(u32, u32)>>>,
    }

    /// A loaded actor with a body node, a skin parent node with two children
    /// (the second one's property name starts with `Skin` and attaches), a
    /// head node that is not set up yet, and a female NPC with a race.
    fn head_world() -> HeadWorld {
        let mut e = engine();
        let npc = new_npc(&mut e);
        give_vtables(&mut e, npc, false, 0);
        let base = npc.addr();
        e.set(npc, TESNPC::iActorBaseFlags, 1);
        e.set(npc, TESNPC::pHair, Ptr::new(0xa1));
        e.set(npc, TESNPC::fHairLength, 0.5);
        e.set(npc, TESNPC::iHairColor, 7);
        e.set(npc, TESNPC::spHeadBiped, Ptr::new(0x9001));
        let race = e.mem.alloc(0x400);
        e.mem.set_u32(race + 0xa8, 0xe1);
        e.set(npc, TESNPC::pFormRace, Ptr::new(race));
        e.mem.set_u8(0x011d_5a30 + 4, 1);
        e.mem.set_u8(0x011c_b99c + 4, 1);
        e.mem.set_u8(0x011d_5adc + 4, 1);
        for i in 0..8 {
            e.mem.set_u32(0x0119_9fc4 + 4 * i, 0x3000 + i);
        }
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER_SINGLETON, player);

        // Scene graph: every node answers virtual 0xc and 0x1c with itself.
        let node = |e: &mut Engine, n: u32| {
            let object = e.mem.alloc(0x40);
            put_object_vtable(
                e,
                object,
                0x0be0_0000 + n * 0x100,
                &[(0xc, 0x0be1_0000), (0x1c, 0x0be1_0000)],
            );
            object
        };
        e.register(0x0be1_0000, |_, a| ret(a[0]));
        let body = node(&mut e, 1);
        let parent = node(&mut e, 2);
        let child_a = node(&mut e, 3);
        let child_b = node(&mut e, 4);
        let name_a = text(&mut e, "Other");
        let name_b = text(&mut e, "SkinTone");
        e.mem.set_u32(child_a + 0x14, name_a);
        e.mem.set_u32(child_b + 0x14, name_b);
        let head = e.mem.alloc(0x40);
        put_object_vtable(
            &mut e,
            head,
            0x0be9_0000,
            &[(0x120, 0x0be1_0001), (0x124, 0x0be1_0002)],
        );
        e.register(0x0be1_0001, |_, _| ret(0));
        e.register(0x0be1_0002, |_, _| ret(0));
        let actor = e.mem.alloc(0x100);
        put_object_vtable(
            &mut e,
            actor,
            0x0bea_0000,
            &[(0x1d0, 0x0be1_0003), (0x1b4, 0x0be1_0004)],
        );
        e.register_double(0x0be1_0003, |_, _| ret(0x8000));
        e.register_double(0x0be1_0004, move |_, _| ret(head));
        e.register_double(FIND_NODE_BY_NAME, move |_, a| {
            ret(match a[1] {
                0x3000 => body,
                SKIN_PARENT_NODE_NAME => parent,
                _ => 0,
            })
        });
        e.register_double(NODE_CHILD_AT, move |_, a| {
            ret([child_a, child_b][a[1] as usize])
        });
        e.register(NODE_CHILD_COUNT, |_, _| ret(2));
        e.register(GET_PROPERTY, |_, a| ret(a[0]));
        e.register(CAST_TO_SHADER_PROPERTY, |_, a| ret(a[1]));
        e.register(0x0041_3f40, |_, a| ret(a[0]));
        e.register(0x0043_b1b0, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(CRT_STRNICMP, |e, a| {
            let first = e.mem.cstr(a[0]);
            let second = e.mem.cstr(a[1]);
            let n = a[2] as usize;
            let lower = |s: &[u8]| {
                s.iter()
                    .take(n)
                    .map(|c| c.to_ascii_lowercase())
                    .collect::<Vec<_>>()
            };
            ret((lower(&first) != lower(&second)) as u32)
        });
        e.set_global(HALF, 0.5f32);
        e.mem.set_cstr(SKIN_PROPERTY_PREFIX, b"Skin");
        e.register(RACE_EYE_LIST, |_, a| ret(a[0] + 0xa8));
        e.register_double(ATTACH_TO_HEAD, move |_, a| ret((a[0] == child_b) as u32));
        e.register(RACE_HAIR_ENTRY, |_, a| ret(0x1000 + a[2]));
        e.register(RACE_HEAD_PART_ENTRY, |_, a| ret(0x2000 + a[2]));
        let applied = Rc::new(RefCell::new(vec![]));
        let sink = applied.clone();
        e.register_double(APPLY_FACE_PARAMS, move |e, a| {
            sink.borrow_mut()
                .push((a[0], e.mem.bytes(a[1], FACE_PARAMS_SIZE)));
            ret(0)
        });
        let words = Rc::new(RefCell::new(vec![]));
        let sink = words.clone();
        e.register_double(ARRAY_APPEND_WORD, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            ret(0)
        });
        let _ = base;
        HeadWorld {
            e,
            npc,
            actor,
            head,
            applied,
            words,
        }
    }

    #[test]
    fn head_rebuild_attaches_the_skin_and_builds_the_face_parameters() {
        let mut w = head_world();
        started(&mut w.e);
        let base = w.npc.addr();
        w.e.call(0x0060_56f0, &args![w.npc, w.actor]);
        // The head node was told it is set up (virtual 0x124, true) after the
        // second child attached.
        assert_eq!(calls(&mut w.e, 0x0be1_0002), vec![vec![w.head, 1]]);
        // The head node is also stored in spHeadSkinned.
        assert_eq!(w.e.get(w.npc, TESNPC::spHeadSkinned), Ptr::new(w.head));
        // The parameter block went to both head nodes.
        let applied = w.applied.borrow();
        assert_eq!(applied.len(), 2);
        assert_eq!(applied[0].0, 0x9001);
        assert_eq!(applied[1].0, w.head);
        let block = &applied[0].1;
        let word =
            |offset: usize| u32::from_le_bytes(block[offset..offset + 4].try_into().unwrap());
        assert_eq!(word(0x80), 0xa1);
        assert_eq!(word(0x84), 7);
        assert_eq!(f32::from_bits(word(0x88)), 0.5);
        // No eye colour of its own: the race's first one.
        assert_eq!(word(0x8c), 0xe1);
        assert_eq!(word(0x90), 1);
        assert_eq!(block[0xd4], 1);
        // Eight entries in each of the three word arrays, in slot order, and
        // eight face-mod textures (cleared by the setting at 011d5adc).
        let appended = w.words.borrow();
        let values = |from: usize| -> Vec<u32> {
            appended
                .iter()
                .skip(from)
                .step_by(3)
                .take(8)
                .map(|(_, v)| *v)
                .collect()
        };
        assert_eq!(appended.len(), 24);
        assert_eq!(values(0), (0..8).map(|i| 0x1000 + i).collect::<Vec<_>>());
        assert_eq!(values(1), (0..8).map(|i| 0x2000 + i).collect::<Vec<_>>());
        assert_eq!(values(2), (0..8).map(|i| 0x3000 + i).collect::<Vec<_>>());
        drop(appended);
        drop(applied);
        assert_eq!(calls(&mut w.e, ARRAY_APPEND_NI_POINTER).len(), 8);
        assert_eq!(calls(&mut w.e, FACE_PARAMS_CONSTRUCT).len(), 1);
        assert_eq!(calls(&mut w.e, FACE_PARAMS_DESTROY).len(), 1);
        let _ = base;
    }

    #[test]
    fn head_rebuild_stops_early_without_setting_actor_race_or_nodes() {
        // The setting is off: nothing at all.
        let mut w = head_world();
        w.e.mem.set_u8(0x011d_5a30 + 4, 0);
        started(&mut w.e);
        w.e.call(0x0060_56f0, &args![w.npc, w.actor]);
        assert_eq!(w.e.call_log.as_ref().unwrap().len(), 2);
        // A null actor: nothing either.
        let mut w = head_world();
        started(&mut w.e);
        w.e.call(0x0060_56f0, &args![w.npc, 0u32]);
        assert_eq!(w.e.call_log.as_ref().unwrap().len(), 1);
        // No body node: the head is stored but nothing is built.
        let mut w = head_world();
        w.e.register(FIND_NODE_BY_NAME, |_, _| ret(0));
        w.e.call(0x0060_56f0, &args![w.npc, w.actor]);
        assert!(w.applied.borrow().is_empty());
        // No race: the skin is attached but no parameters are built.
        let mut w = head_world();
        w.e.set(w.npc, TESNPC::pFormRace, Ptr::NULL);
        w.e.call(0x0060_56f0, &args![w.npc, w.actor]);
        assert!(w.applied.borrow().is_empty());
        // A head node that reports itself already set up is not touched, and
        // the eyes of the NPC are used when it has some.
        let mut w = head_world();
        w.e.register(0x0be1_0001, |_, _| ret(1));
        w.e.set(w.npc, TESNPC::pEyeColor, Ptr::new(0xe2));
        started(&mut w.e);
        w.e.call(0x0060_56f0, &args![w.npc, w.actor]);
        assert!(calls(&mut w.e, 0x0be1_0002).is_empty());
        let applied = w.applied.borrow();
        assert_eq!(
            u32::from_le_bytes(applied[0].1[0x8c..0x90].try_into().unwrap()),
            0xe2
        );
    }

    #[test]
    fn head_rebuild_searches_the_first_childs_children_when_there_is_one() {
        let mut w = head_world();
        // The skin parent has a first child; the search goes through its
        // children instead of the parent's.
        w.e.register(NODE_FIRST_CHILD, |_, a| ret(a[0]));
        started(&mut w.e);
        w.e.call(0x0060_56f0, &args![w.npc, w.actor]);
        assert_eq!(calls(&mut w.e, 0x0be1_0002), vec![vec![w.head, 1]]);
        // That first child answers virtual 0xc with itself: the child counts
        // asked for are of the same node.
        assert!(!calls(&mut w.e, NODE_CHILD_COUNT).is_empty());
    }

    /// A scripted `TESFile`: the chunks of one record and the answers the
    /// loader asks for.
    struct FakeFile {
        chunks: Vec<(u32, Vec<u8>)>,
        position: usize,
        version: u16,
        swap: bool,
        form_type: u8,
        header: u32,
    }

    fn word_bytes(v: u32) -> Vec<u8> {
        v.to_le_bytes().to_vec()
    }

    fn float_bytes(v: f32) -> Vec<u8> {
        v.to_le_bytes().to_vec()
    }

    /// An NPC, a file object and the doubles that make the file read the
    /// given chunks. `FAKE_NOOP` doubles stand behind the NPC's virtual
    /// slots; `speed` is the speed multiplier the actor-base data reports.
    fn load_world(
        chunks: Vec<(u32, Vec<u8>)>,
    ) -> (Engine, Ptr<TESNPC>, u32, Rc<RefCell<FakeFile>>) {
        let mut e = engine();
        let this = new_npc(&mut e);
        give_vtables(&mut e, this, false, 0);
        e.register(0x008f_21d0, |_, _| ret(100));
        let file = e.mem.alloc(0x100);
        e.mem.set_cstr(file + 0x20, b"FalloutNV.esm");
        let state = Rc::new(RefCell::new(FakeFile {
            chunks,
            position: 0,
            version: 12,
            swap: false,
            form_type: 0x2a,
            header: 0,
        }));
        let s = state.clone();
        e.register_double(FILE_GET_FORM_TYPE, move |_, _| {
            ret(s.borrow().form_type as u32)
        });
        e.register(FILE_RECORD_OFFSET, |_, _| ret(0x1234_5678));
        let s = state.clone();
        e.register_double(FILE_GET_CHUNK_TAG, move |_, _| {
            let s = s.borrow();
            ret(s.chunks.get(s.position).map_or(0, |c| c.0))
        });
        let s = state.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.position += 1;
            ret((s.position < s.chunks.len()) as u32)
        });
        let s = state.clone();
        e.register_double(FILE_CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            ret(s.chunks[s.position].1.len() as u32)
        });
        let s = state.clone();
        e.register_double(FILE_NEEDS_SWAP, move |_, _| ret(s.borrow().swap as u32));
        let s = state.clone();
        e.register_double(FILE_VERSION, move |_, _| ret(s.borrow().version as u32));
        let s = state.clone();
        e.register_double(FORM_DATA_HEADER_SIZE, move |_, _| ret(s.borrow().header));
        for (addr, limit) in [(FILE_GET_CHUNK_WORD, 4usize), (FILE_GET_CHUNK_U16, 2)] {
            let s = state.clone();
            e.register_double(addr, move |e, a| {
                let s = s.borrow();
                let data = &s.chunks[s.position].1;
                e.mem.write(a[1], &data[..data.len().min(limit)]);
                ret(0)
            });
        }
        let s = state.clone();
        e.register_double(FILE_GET_CHUNK_BYTES, move |e, a| {
            let s = s.borrow();
            let data = &s.chunks[s.position].1;
            e.mem.write(a[1], &data[..data.len().min(a[2] as usize)]);
            ret(0)
        });
        // The record's form id and name for the log messages.
        e.set(this, TESNPC::iFormID, 0x0100_0abc);
        (e, this, file, state)
    }

    fn chunk(name: &[u8; 4], data: Vec<u8>) -> (u32, Vec<u8>) {
        (chunk_tag(name), data)
    }

    fn load(e: &mut Engine, this: Ptr<TESNPC>, file: u32) -> bool {
        e.call(0x0060_2190, &args![this, file]).bool()
    }

    #[test]
    fn load_leaves_other_records_alone() {
        let (mut e, this, file, state) = load_world(vec![chunk(b"CNAM", word_bytes(5))]);
        state.borrow_mut().form_type = 0x10;
        started(&mut e);
        assert!(!load(&mut e, this, file));
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.get(this, TESNPC::pCl), Ptr::NULL);
    }

    #[test]
    fn load_reads_the_simple_chunks_into_their_fields() {
        let (mut e, this, file, _) = load_world(vec![
            chunk(b"CNAM", word_bytes(0x1234)),
            chunk(b"NAM4", word_bytes(3)),
            chunk(b"NAM6", float_bytes(1.8)),
            chunk(b"NAM7", float_bytes(70.0)),
            chunk(b"NAM9", word_bytes(0x00aa_bbcc)),
            chunk(b"TPLT", word_bytes(0x7e)),
            chunk(b"VTCK", word_bytes(0x7f)),
            chunk(b"INAM", word_bytes(0x80)),
            chunk(b"RNAM", word_bytes(0x77)),
            chunk(b"ZNAM", word_bytes(0x88)),
            chunk(b"SCRI", word_bytes(0x99)),
            chunk(b"EITM", word_bytes(0x55)),
            chunk(b"SPLO", word_bytes(0x66)),
            chunk(b"EAMT", vec![5, 0]),
            chunk(b"PKID", word_bytes(0x44)),
            chunk(b"ABCD", vec![1, 2, 3]),
        ]);
        let base = this.addr();
        started(&mut e);
        assert!(load(&mut e, this, file));
        assert_eq!(e.get(this, TESNPC::iFileOffset), 0x1234_5678);
        assert_eq!(calls(&mut e, FORM_LOAD_FORM), vec![vec![base, file]]);
        assert_eq!(calls(&mut e, 0x0048_4ab0), vec![vec![base, 0]]);
        assert_eq!(e.get(this, TESNPC::pCl), Ptr::new(0x1234));
        assert_eq!(e.get(this, TESNPC::eBloodImpactMaterial), 3);
        assert_eq!(e.get(this, TESNPC::fHeight), 1.8);
        assert_eq!(e.get(this, TESNPC::fWeight), 70.0);
        assert_eq!(e.get(this, TESNPC::iHairColor), 0x00aa_bbcc);
        assert_eq!(e.mem.u32(base + 0x54), 0x7e);
        assert_eq!(e.mem.u32(base + 0x50), 0x7f);
        assert_eq!(e.mem.u32(base + 0x4c), 0x80);
        assert_eq!(e.get(this, TESNPC::pFormRace), Ptr::new(0x77));
        assert_eq!(e.get(this, TESNPC::pCombatStyle), Ptr::new(0x88));
        assert_eq!(e.mem.u32(base + COMPONENT_SCRIPTABLE + 4), 0x99);
        assert_eq!(
            calls(&mut e, SCRIPTABLE_INIT_ITEM),
            vec![vec![base + COMPONENT_SCRIPTABLE, base]]
        );
        assert_eq!(e.mem.u32(base + COMPONENT_TOUCH_SPELL + 4), 0x55);
        assert_eq!(
            calls(&mut e, SPELL_LIST_ADD_SPELL),
            vec![vec![base + COMPONENT_SPELL_LIST, 0x66]]
        );
        assert_eq!(
            calls(&mut e, TOUCH_SPELL_SET_ANIMATION),
            vec![vec![base + COMPONENT_TOUCH_SPELL, 5]]
        );
        assert_eq!(
            calls(&mut e, AI_FORM_ADD_PACKAGE),
            vec![vec![base + COMPONENT_AI_FORM, 0x44]]
        );
    }

    #[test]
    fn load_hands_component_chunks_to_their_components() {
        let (mut e, this, file, _) = load_world(vec![
            chunk(b"DEST", vec![]),
            chunk(b"DSTD", vec![]),
            chunk(b"FULL", vec![]),
            chunk(b"MODL", vec![]),
            chunk(b"MODT", vec![]),
            chunk(b"KFFZ", vec![]),
            chunk(b"OBND", vec![]),
        ]);
        let base = this.addr();
        started(&mut e);
        assert!(load(&mut e, this, file));
        assert_eq!(
            calls(&mut e, DESTRUCTIBLE_LOAD_CHUNK),
            vec![vec![base + COMPONENT_DESTRUCTIBLE, file]; 2]
        );
        assert_eq!(
            calls(&mut e, FULL_NAME_LOAD),
            vec![vec![base + COMPONENT_FULL_NAME, file]]
        );
        assert_eq!(
            calls(&mut e, MODEL_LOAD_CHUNK),
            vec![vec![base + COMPONENT_MODEL, file]; 2]
        );
        assert_eq!(
            calls(&mut e, ANIMATION_LOAD),
            vec![vec![
                base + COMPONENT_ANIMATION,
                base + COMPONENT_ANIMATION,
                file
            ]]
        );
        // OBND goes through the NPC's virtual 0xe0 (a recorded no-op here).
        assert!(calls(&mut e, FAKE_NOOP).contains(&vec![base, file]));
    }

    #[test]
    fn load_sets_the_editor_id_from_the_chunk() {
        let (mut e, this, file, _) = load_world(vec![chunk(b"EDID", b"VulpesInculta\0".to_vec())]);
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.mem.set_u32(FAKE_VTABLE + 0x134, 0x0ba2_0030);
        e.register_double(0x0ba2_0030, move |e, a| {
            sink.borrow_mut().push(e.mem.cstr(a[1]));
            ret(0)
        });
        assert!(load(&mut e, this, file));
        assert_eq!(*seen.borrow(), vec![b"VulpesInculta".to_vec()]);
    }

    #[test]
    fn load_looks_up_hair_eyes_and_head_parts_and_logs_missing_ones() {
        let (mut e, this, file, _) = load_world(vec![
            chunk(b"HNAM", word_bytes(0xa1)),
            chunk(b"ENAM", word_bytes(0xe1)),
            chunk(b"PNAM", word_bytes(0xb1)),
        ]);
        let base = this.addr();
        // File indices are added to the ids; every form is found.
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            ret(0)
        });
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        started(&mut e);
        assert!(load(&mut e, this, file));
        assert_eq!(e.get(this, TESNPC::pHair), Ptr::new(0x0100_00a1));
        assert_eq!(e.get(this, TESNPC::pEyeColor), Ptr::new(0x0100_00e1));
        let added = calls(&mut e, 0x005a_e3d0);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], base + 0x1dc);
        assert_eq!(e.mem.u32(added[0][1]), 0x0100_00b1);
        assert!(calls(&mut e, LOG_MESSAGE).is_empty());

        // None of them exist: each is reported with its id, the NPC's name
        // and form id, and nothing is stored.
        let (mut e, this, file, _) = load_world(vec![
            chunk(b"HNAM", word_bytes(0xa1)),
            chunk(b"ENAM", word_bytes(0xe1)),
            chunk(b"PNAM", word_bytes(0xb1)),
        ]);
        e.register(LOOKUP_FORM, |_, _| ret(0));
        started(&mut e);
        assert!(load(&mut e, this, file));
        assert_eq!(e.get(this, TESNPC::pHair), Ptr::NULL);
        assert_eq!(e.get(this, TESNPC::pEyeColor), Ptr::NULL);
        assert!(calls(&mut e, 0x005a_e3d0).is_empty());
        let logged = calls(&mut e, LOG_MESSAGE);
        let name = 0x0777_0000;
        assert_eq!(
            &logged[..3],
            &[
                vec![MESSAGE_NO_HAIR, 0xa1, name, 0x0100_0abc],
                vec![MESSAGE_NO_EYES, 0xe1, name, 0x0100_0abc],
                vec![MESSAGE_NO_HEAD_PART, 0xb1, name, 0x0100_0abc],
            ]
        );
    }

    #[test]
    fn load_clamps_the_hair_length() {
        let (mut e, this, file, _) = load_world(vec![
            chunk(b"LNAM", float_bytes(-0.5)),
            chunk(b"LNAM", float_bytes(0.25)),
            chunk(b"LNAM", float_bytes(3.0)),
        ]);
        started(&mut e);
        assert!(load(&mut e, this, file));
        let lengths: Vec<f32> = calls(&mut e, NPC_SET_HAIR_LENGTH)
            .iter()
            .map(|c| f32::from_bits(c[1]))
            .collect();
        assert_eq!(lengths, vec![0.0, 0.25, 1.0]);
    }

    #[test]
    fn load_reads_the_npc_data_in_each_of_its_sizes() {
        let (mut e, this, file, _) = load_world(vec![
            chunk(b"DATA", vec![]),
            chunk(b"DATA", vec![0; 0x1c]),
            chunk(b"DATA", vec![0; 0xe]),
            chunk(b"DATA", vec![0; 5]),
            chunk(b"DNAM", (1..=0x1c).collect()),
        ]);
        let base = this.addr();
        started(&mut e);
        assert!(load(&mut e, this, file));
        assert_eq!(
            calls(&mut e, FORM_LOAD_DATA),
            vec![
                vec![base, file, 0, 0],
                vec![base, file, base + 0x114, 0x1c],
                vec![base, file, base + 0x114, 0xe],
                vec![base, file, base + 0x114, 5],
            ]
        );
        // Only the odd size (5) is reported.
        assert_eq!(
            calls(&mut e, LOG_MESSAGE),
            vec![vec![MESSAGE_INVALID_DATA_FORMAT, 0x0777_0000, 0x0100_0abc]]
        );
        // DNAM is the same data as a plain 0x1c-byte read.
        assert_eq!(
            e.mem.bytes(base + 0x114, 0x1c),
            (1..=0x1c).collect::<Vec<u8>>()
        );
    }

    #[test]
    fn load_reads_the_actor_base_data_and_clears_old_template_flags() {
        let data: Vec<u8> = (1..=0x18).collect();
        let (mut e, this, file, state) = load_world(vec![chunk(b"ACBS", data)]);
        let base = this.addr();
        state.borrow_mut().version = 5;
        state.borrow_mut().swap = true;
        started(&mut e);
        assert!(load(&mut e, this, file));
        let loaded = e.mem.bytes(base + 0x34, 0x18);
        let mut want: Vec<u8> = (1..=0x18).collect();
        // The 16-bit values at +6 (bartergold) and +0x16 (template flags,
        // cleared for versions before 8) are zero.
        want[6] = 0;
        want[7] = 0;
        want[0x16] = 0;
        want[0x17] = 0;
        assert_eq!(loaded, want);
        assert_eq!(calls(&mut e, SWAP_ACTOR_BASE_DATA), vec![vec![base + 0x34]]);

        // A current file keeps the template flags.
        let (mut e, this, file, _) = load_world(vec![chunk(b"ACBS", (1..=0x18).collect())]);
        assert!(load(&mut e, this, file));
        let loaded = e.mem.bytes(this.addr() + 0x34, 0x18);
        assert_eq!(&loaded[0x16..], &[0x17, 0x18]);
        assert_eq!(&loaded[6..8], &[0, 0]);
    }

    #[test]
    fn load_converts_old_ai_data() {
        let mut data = vec![0u8; 0x14];
        data[0] = 3;
        data[1] = 1;
        let (mut e, this, file, state) = load_world(vec![chunk(b"AIDT", data.clone())]);
        let base = this.addr();
        state.borrow_mut().version = 3;
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(AI_FORM_SET_DATA, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.bytes(a[1], 0x14)));
            ret(0)
        });
        assert!(load(&mut e, this, file));
        // Version 3: first byte 3 -> 4 -> (below 6) 2 with +0xe = 2; the second
        // byte is mirrored (4 - 1 = 3).
        let seen = seen.borrow();
        assert_eq!(seen[0].0, base + COMPONENT_AI_FORM);
        assert_eq!(seen[0].1[0], 2);
        assert_eq!(seen[0].1[1], 3);
        assert_eq!(seen[0].1[0xe], 2);

        // Version 5 maps each old value; version 7 changes nothing.
        let cases = [
            (5u16, 0u8, 0u8, 0u8),
            (5, 1, 0, 1),
            (5, 2, 1, 1),
            (5, 3, 1, 2),
            (5, 4, 2, 2),
            (5, 5, 3, 0),
            (7, 4, 4, 0),
        ];
        for (version, old, first, extra) in cases {
            let mut data = vec![0u8; 0x14];
            data[0] = old;
            data[1] = 1;
            let (mut e, this, file, state) = load_world(vec![chunk(b"AIDT", data)]);
            state.borrow_mut().version = version;
            let seen = Rc::new(RefCell::new(vec![]));
            let sink = seen.clone();
            e.register_double(AI_FORM_SET_DATA, move |e, a| {
                sink.borrow_mut().push(e.mem.bytes(a[1], 0x14));
                ret(0)
            });
            assert!(load(&mut e, this, file));
            let bytes = seen.borrow()[0].clone();
            assert_eq!(
                (bytes[0], bytes[0xe]),
                (first, extra),
                "version {version} old {old}"
            );
            // The second byte is mirrored before version 7 only.
            assert_eq!(bytes[1], if version < 7 { 3 } else { 1 });
        }
    }

    #[test]
    fn load_reads_race_face_number_factions_and_the_swapped_values() {
        let (mut e, this, file, state) = load_world(vec![
            chunk(b"FNAM", vec![0x34, 0x12]),
            chunk(b"FNAM", vec![0x99]),
            chunk(b"SNAM", vec![0xfa, 0, 0, 0, 3, 0, 0, 0]),
        ]);
        let base = this.addr();
        state.borrow_mut().swap = true;
        started(&mut e);
        assert!(load(&mut e, this, file));
        // The one-byte FNAM is ignored; the two-byte one is swapped.
        assert_eq!(e.get(this, TESNPC::sLastRaceFaceNum), 0x1234);
        assert_eq!(calls(&mut e, SWAP_U16), vec![vec![base + 0x1d0, 0]]);
        assert_eq!(calls(&mut e, SWAP_FACTION_ENTRY).len(), 1);
        assert_eq!(
            calls(&mut e, SET_FACTION_RANK),
            vec![vec![base + COMPONENT_ACTOR_BASE_DATA, 0xfa, 3]]
        );
    }

    #[test]
    fn load_reads_face_gen_coordinates_into_the_race_matrices() {
        let mut chunks = vec![];
        for (tag, values) in [
            (b"FGGS", [1.0f32, 2.0]),
            (b"FGGA", [3.0, 4.0]),
            (b"FGTS", [5.0, 6.0]),
        ] {
            let mut data = float_bytes(values[0]);
            data.extend(float_bytes(values[1]));
            chunks.push(chunk(tag, data));
        }
        let (mut e, this, file, state) = load_world(chunks);
        let base = this.addr();
        state.borrow_mut().swap = true;
        // A fake vector: resize makes a buffer, the iterator remembers the
        // matrix and index, the dereference finds the element.
        let buffers = Rc::new(RefCell::new(std::collections::HashMap::new()));
        let sink = buffers.clone();
        e.register_double(MATRIX_RESIZE, move |e, a| {
            let buffer = e.mem.alloc(a[1] * 4);
            sink.borrow_mut().insert(a[0], buffer);
            ret(0)
        });
        e.register(MATRIX_ITERATOR_AT, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            e.mem.set_u32(a[1] + 4, a[2]);
            ret(a[1])
        });
        let lookup = buffers.clone();
        e.register_double(ITERATOR_ELEMENT, move |e, a| {
            let matrix = e.mem.u32(a[0]);
            let index = e.mem.u32(a[0] + 4);
            ret(lookup.borrow()[&matrix] + index * 4)
        });
        started(&mut e);
        assert!(load(&mut e, this, file));
        let value =
            |e: &Engine, matrix: u32, index: u32| e.mem.f32(buffers.borrow()[&matrix] + index * 4);
        let m00 = base + 0x134;
        let m01 = base + 0x134 + 0x20;
        let m10 = base + 0x134 + 0x40;
        assert_eq!((value(&e, m00, 0), value(&e, m00, 1)), (1.0, 2.0));
        assert_eq!((value(&e, m01, 0), value(&e, m01, 1)), (3.0, 4.0));
        assert_eq!((value(&e, m10, 0), value(&e, m10, 1)), (5.0, 6.0));
        // Each matrix was resized to the float count, and the swap helper ran
        // for each float.
        assert_eq!(
            calls(&mut e, MATRIX_RESIZE),
            vec![vec![m00, 2, 1], vec![m01, 2, 1], vec![m10, 2, 1]]
        );
        assert_eq!(calls(&mut e, SWAP_WORD).len(), 6);
    }

    #[test]
    fn load_builds_container_entries_and_their_extra_data() {
        let (mut e, this, file, _) = load_world(vec![
            chunk(b"COED", vec![1, 2, 3, 4]),
            chunk(b"CNTO", vec![0x11, 0, 0, 0, 2, 0, 0, 0]),
            chunk(b"COED", vec![1, 2, 3, 4]),
        ]);
        let base = this.addr();
        let item = e.mem.alloc(0x20);
        e.register_double(CONTAINER_ADD_ENTRY, move |_, _| ret(item));
        e.register(CONTAINER_EXTRA_CONSTRUCT, |_, a| ret(a[0]));
        started(&mut e);
        assert!(load(&mut e, this, file));
        // The first COED has nothing to extend; the CNTO adds the entry (the
        // 8 bytes read), the second COED attaches extra data to it and ends
        // the pending item.
        let adds = calls(&mut e, CONTAINER_ADD_ENTRY);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], base + COMPONENT_CONTAINER);
        let extra = e.mem.u32(item + 8);
        assert_ne!(extra, 0);
        assert_eq!(calls(&mut e, CONTAINER_EXTRA_LOAD), vec![vec![extra, file]]);
        assert_eq!(e.global::<u32>(CONTAINER_ITEM_BEING_LOADED), 0);
    }

    #[test]
    fn load_reports_face_texture_chunks_once() {
        let (mut e, this, file, _) = load_world(vec![
            chunk(b"NAM0", vec![]),
            chunk(b"NAM2", vec![]),
            chunk(b"NAM3", vec![]),
        ]);
        started(&mut e);
        assert!(load(&mut e, this, file));
        let logged = calls(&mut e, LOG_MESSAGE);
        let nonzero: Vec<_> = logged
            .iter()
            .filter(|c| c[0] == MESSAGE_FACE_TEXTURE_FOUND)
            .collect();
        assert_eq!(nonzero.len(), 1);
        assert_eq!(&nonzero[0][1..], &[0x0777_0000, 0x0100_0abc, file + 0x20]);
    }

    #[test]
    fn load_finishes_with_the_player_reference_and_the_speed_check() {
        // The player's base form (form id 7) is given to a player reference
        // that has none; a zero speed multiplier is reported.
        let (mut e, this, file, _) = load_world(vec![chunk(b"CNAM", word_bytes(1))]);
        e.set(this, TESNPC::iFormID, PLAYER_BASE_FORM_ID);
        e.register(0x008f_21d0, |_, _| ret(0));
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER_SINGLETON, player);
        started(&mut e);
        assert!(load(&mut e, this, file));
        assert_eq!(
            calls(&mut e, REFERENCE_SET_OBJECT_REFERENCE),
            vec![vec![player, this.addr()]]
        );
        assert_eq!(
            calls(&mut e, LOG_MESSAGE),
            vec![vec![MESSAGE_SPEED_ZERO, 0x0777_0000, PLAYER_BASE_FORM_ID]]
        );
        // A player reference that already has a base form is left alone.
        let (mut e, this, file, _) = load_world(vec![chunk(b"CNAM", word_bytes(1))]);
        e.set(this, TESNPC::iFormID, PLAYER_BASE_FORM_ID);
        let player = e.mem.alloc(0x100);
        e.mem.set_u32(player + 0x20, 0x1000);
        e.set_global(PLAYER_SINGLETON, player);
        started(&mut e);
        assert!(load(&mut e, this, file));
        assert!(calls(&mut e, REFERENCE_SET_OBJECT_REFERENCE).is_empty());
        assert!(calls(&mut e, LOG_MESSAGE).is_empty());
    }

    // BEGIN second-block tests

    // -----------------------------------------------------------------------
    // Second block: `ReplaceRefModel` (00605d70) to 0060b1f0

    /// An engine where every callee of the second block is a double
    /// returning 0 (and recording its call), besides those `engine()` gives
    /// real bodies.
    fn world() -> Engine {
        let mut e = engine();
        for addr in SECOND_CALLEES {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    /// Makes the function at `addr` a double that returns `value`.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    /// An object whose vtable is at `table`; each `(offset, value)` slot is a
    /// double (at `table + 0x1000 + offset`) that returns `value`.
    fn object_with(e: &mut Engine, table: u32, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x400);
        let size = slots.iter().map(|s| s.0).max().unwrap_or(0) + 4;
        e.map(table, size);
        for &(offset, value) in slots {
            let target = table + 0x1000 + offset;
            e.mem.set_u32(table + offset, target);
            returns(e, target, value);
        }
        e.mem.set_u32(object, table);
        object
    }

    /// The address of the double `object_with` made for a slot.
    fn slot(table: u32, offset: u32) -> u32 {
        table + 0x1000 + offset
    }

    /// An object of `size` bytes whose words are `words`.
    fn block(e: &mut Engine, size: u32, words: &[(u32, u32)]) -> u32 {
        let block = e.mem.alloc(size);
        for &(offset, value) in words {
            e.mem.set_u32(block + offset, value);
        }
        block
    }

    #[test]
    fn replace_ref_model_rebuilds_once_for_an_actor_and_twice_for_the_player() {
        let mut e = world();
        let this = new_npc(&mut e);
        let biped = e.mem.alloc(0x400);
        let node = object_with(&mut e, 0x0c00_0000, &[(0xbc, 0)]);
        let actor = object_with(&mut e, 0x0c01_0000, &[(0x1e8, biped)]);
        returns(&mut e, 0x0043_fcd0, node);
        started(&mut e);
        e.call(0x0060_5d70, &args![this, actor]);
        assert_eq!(calls(&mut e, 0x004a_c1e0), vec![vec![biped, 0]]);
        assert_eq!(calls(&mut e, 0x00a5_a040), vec![vec![node]]);
        assert_eq!(calls(&mut e, slot(0x0c00_0000, 0xbc)), vec![vec![node]]);
        let update = calls(&mut e, 0x00a5_9c60);
        assert_eq!(update.len(), 1);
        assert_eq!(update[0][0], node);
        // The player: a second round with the first-person biped and node.
        let mut e = world();
        let this = new_npc(&mut e);
        let biped = e.mem.alloc(0x400);
        let node = object_with(&mut e, 0x0c00_0000, &[(0xbc, 0)]);
        let first_biped = e.mem.alloc(0x400);
        let first_node = object_with(&mut e, 0x0c02_0000, &[(0xbc, 0)]);
        let player = object_with(&mut e, 0x0c01_0000, &[(0x1e8, biped)]);
        e.set_global(PLAYER_SINGLETON, player);
        returns(&mut e, 0x0043_fcd0, node);
        returns(&mut e, 0x004e_af60, 1);
        returns(&mut e, 0x0095_0b00, first_biped);
        returns(&mut e, 0x0095_0bb0, first_node);
        started(&mut e);
        e.call(0x0060_5d70, &args![this, player]);
        assert_eq!(
            calls(&mut e, 0x004a_c1e0),
            vec![vec![biped, 0], vec![first_biped, 0]]
        );
        assert_eq!(
            calls(&mut e, 0x00a5_a040),
            vec![vec![node], vec![first_node]]
        );
        assert_eq!(calls(&mut e, 0x0095_0b00), vec![vec![player, 1]; 3]);
    }

    #[test]
    fn worn_items_are_put_on_the_biped_and_the_models_refreshed() {
        let mut e = world();
        let this = new_npc(&mut e);
        e.set(this, TESNPC::iActorBaseFlags, 1);
        e.mem.set_u32(this.addr() + COMPONENT_RACE + 4, 0x7ace);
        let biped = e.mem.alloc(0x400);
        let node = object_with(&mut e, 0x0c02_0000, &[(0xbc, 0)]);
        let process = object_with(&mut e, 0x0c03_0000, &[(0x468, 0), (0x470, 0)]);
        let actor = object_with(&mut e, 0x0c04_0000, &[(0x1d0, node)]);
        e.mem.set_u32(actor + 0x68, process);
        let items = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        for item in items {
            e.mem.set_u8(item + 4, 0x28);
        }
        let cells = block(&mut e, 8, &[(0, items[0]), (4, items[1])]);
        returns(&mut e, 0x0044_ddc0, 2);
        e.register_double(0x006a_7ad0, move |_, a| ret(cells + a[1] * 4));
        let worn = e.mem.alloc(0x20);
        started(&mut e);
        e.call(0x0060_5e70, &args![this, actor, biped, worn]);
        assert_eq!(calls(&mut e, 0x004a_b250), vec![vec![biped, 0x7ace, 1]]);
        assert_eq!(
            calls(&mut e, 0x004a_b400),
            vec![vec![biped, items[0], 0], vec![biped, items[1], 0]]
        );
        assert_eq!(
            calls(&mut e, slot(0x0c03_0000, 0x468)),
            vec![vec![process, 1]]
        );
        assert_eq!(calls(&mut e, slot(0x0c03_0000, 0x470)), vec![vec![process]]);
        assert_eq!(calls(&mut e, 0x004a_c1e0), vec![vec![biped, 1]]);
        assert_eq!(calls(&mut e, 0x00a5_a040), vec![vec![node]]);
        assert_eq!(calls(&mut e, slot(0x0c02_0000, 0xbc)), vec![vec![node]]);
    }

    #[test]
    fn build_object_array_adds_each_new_part_that_answers_yes() {
        let mut e = world();
        let this = new_npc(&mut e);
        let yes = object_with(&mut e, 0x0c00_0000, &[(0xe4, 1)]);
        let no = object_with(&mut e, 0x0c01_0000, &[(0xe4, 0)]);
        let duplicate = object_with(&mut e, 0x0c02_0000, &[(0xe4, 1)]);
        let cells = block(&mut e, 0x50, &[(0, yes), (4, no), (8, duplicate)]);
        e.register_double(0x0043_f220, move |_, a| ret(cells + a[1] * 4));
        e.register_double(0x0099_62f0, move |e, a| {
            ret((e.mem.u32(a[1]) == duplicate) as u32)
        });
        let added = Rc::new(RefCell::new(vec![]));
        let seen = added.clone();
        e.register_double(0x007c_b2e0, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let array = e.mem.alloc(0x10);
        e.call(0x0060_5fc0, &args![this, 0u32, 0xdeadu32, array]);
        assert_eq!(*added.borrow(), vec![(array, yes)]);
    }

    /// An actor-base inventory of `entries` (slot, entry) for the
    /// `InitWorn` tests; returns (actor, biped, changes).
    fn worn_actor(e: &mut Engine, entries: &[(u32, u32)]) -> (u32, u32, u32) {
        let actor = e.mem.alloc(0x100);
        let biped = e.mem.alloc(0x400);
        let changes = 0x5000u32;
        returns(e, 0x004b_f220, changes);
        let map: Vec<(u32, u32)> = entries.to_vec();
        e.register_double(0x004c_8c10, move |_, a| {
            ret(map.iter().find(|m| m.0 == a[1]).map_or(0, |m| m.1))
        });
        (actor, biped, changes)
    }

    #[test]
    fn init_worn_visits_the_slots_in_the_exes_order_and_equips_what_is_worn() {
        let mut e = world();
        let this = new_npc(&mut e);
        let item = e.mem.alloc(0x20);
        e.mem.set_u8(item + 4, 0x28);
        let entry = block(&mut e, 0x20, &[(8, item)]);
        let (actor, biped, changes) = worn_actor(&mut e, &[(3, entry)]);
        started(&mut e);
        e.call(0x0060_6050, &args![this, actor, biped]);
        let slots: Vec<u32> = calls(&mut e, 0x004c_8c10).iter().map(|c| c[1]).collect();
        let mut expected = vec![0, 1, 2, 4];
        expected.extend(6..=0x13);
        expected.extend([3, 5]);
        assert_eq!(slots, expected);
        assert!(calls(&mut e, 0x004c_8c10)
            .iter()
            .all(|c| c[0] == changes && c[2] == 0));
        assert_eq!(calls(&mut e, 0x0044_59e0), vec![vec![entry, 1]]);
        assert_eq!(calls(&mut e, 0x004a_b400), vec![vec![biped, item, 0]]);
        // Without an actor or a biped nothing is looked at.
        let mut e = world();
        let this = new_npc(&mut e);
        let (actor, _, _) = worn_actor(&mut e, &[]);
        started(&mut e);
        e.call(0x0060_6050, &args![this, actor, 0u32]);
        assert!(calls(&mut e, 0x004c_8c10).is_empty());
        assert_eq!(calls(&mut e, 0x004b_f220), vec![vec![actor]]);
    }

    #[test]
    fn init_worn_object_adds_models_by_form_type_and_walks_model_lists() {
        let mut e = world();
        let this = new_npc(&mut e);
        e.set(this, TESNPC::iActorBaseFlags, 1);
        let biped = e.mem.alloc(0x400);
        // A weapon (type 0x28) goes to the biped's own function.
        let weapon = e.mem.alloc(0x20);
        e.mem.set_u8(weapon + 4, 0x28);
        started(&mut e);
        assert!(e
            .call(0x0060_61b0, &args![this, 0u32, biped, weapon])
            .bool());
        assert_eq!(calls(&mut e, 0x004a_b400), vec![vec![biped, weapon, 0]]);
        assert!(calls(&mut e, 0x0048_0bd0).is_empty());
        // An item with a biped model: added with this NPC's sex and -1.
        let item = e.mem.alloc(0x20);
        returns(&mut e, 0x0048_0db0, 0x6d6d);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0060_61b0, &args![this, 0u32, biped, item]).bool());
        assert_eq!(
            calls(&mut e, 0x0048_0bd0),
            vec![vec![0x6d6d, biped, 1, u32::MAX]]
        );
        // An item without a model is logged by name.
        returns(&mut e, 0x0048_0db0, 0);
        returns(&mut e, 0x0048_2720, 0x4040);
        e.call_log = Some(vec![]);
        e.call(0x0060_61b0, &args![this, 0u32, biped, item]);
        assert_eq!(calls(&mut e, 0x005b_5e40), vec![vec![0x0104_a828, 0x4040]]);
        // A model list: its models are added until an empty node.
        let first_form = e.mem.alloc(0x20);
        let second_form = e.mem.alloc(0x20);
        let node_two = block(&mut e, 0x10, &[(0, second_form), (4, 0)]);
        let node_one = block(&mut e, 0x10, &[(0, first_form), (4, node_two)]);
        let list = block(&mut e, 0x10, &[(4, 0x1111)]);
        returns(&mut e, 0x0047_5020, list);
        returns(&mut e, 0x0050_0940, node_one);
        e.register_double(0x0048_0db0, |_, a| {
            ret(if a[0] == 0 { 0 } else { a[0] + 0x1000 })
        });
        e.register_double(LIST_NODE_IS_EMPTY, move |_, a| {
            ret((a[0] == node_two) as u32)
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0060_61b0, &args![this, 0u32, biped, item]).bool());
        assert_eq!(
            calls(&mut e, 0x0048_0bd0),
            vec![
                vec![item + 0x1000, biped, 1, u32::MAX],
                vec![first_form + 0x1000, biped, 1, u32::MAX],
            ]
        );
    }

    /// The nodes `fn_006062e0` looks for.
    struct FaceNodes {
        root: u32,
        biped: u32,
        skinned: u32,
        head: u32,
        parts: [u32; 2],
    }

    fn face_nodes(e: &mut Engine, worn: &[(u32, u32)]) -> FaceNodes {
        let nodes = FaceNodes {
            root: e.mem.alloc(0x20),
            biped: e.mem.alloc(0x20),
            skinned: e.mem.alloc(0x20),
            head: e.mem.alloc(0x20),
            parts: [e.mem.alloc(0x20), e.mem.alloc(0x20)],
        };
        e.mem.set_u32(0x0119_9fc4 + 4, 0x4001);
        let (biped, skinned, head, parts) = (nodes.biped, nodes.skinned, nodes.head, nodes.parts);
        e.register_double(FIND_NODE_BY_NAME, move |_, a| {
            ret(match a[1] {
                0x0102_0408 => biped,
                0x0102_03f0 => skinned,
                0x4001 => head,
                0x5000 => parts[0],
                0x5001 => parts[1],
                _ => 0,
            })
        });
        e.register_double(0x0065_7820, |_, a| ret(0x5000 + a[1]));
        returns(e, 0x004b_f220, 0x6000);
        let worn: Vec<(u32, u32)> = worn.to_vec();
        e.register_double(0x004c_8c10, move |_, a| {
            ret(worn.iter().find(|w| w.0 == a[1]).map_or(0, |w| w.1))
        });
        nodes
    }

    #[test]
    fn face_nodes_are_hidden_for_a_worn_body_and_otherwise_follow_the_head_slots() {
        // Slot 0 worn: both face-gen nodes get 1 and the item is released.
        let mut e = world();
        let this = new_npc(&mut e);
        let nodes = face_nodes(&mut e, &[(0, 0x7100)]);
        started(&mut e);
        e.call(0x0060_62e0, &args![this, 0u32, nodes.root]);
        assert_eq!(
            calls(&mut e, 0x0045_0f90),
            vec![vec![nodes.biped, 1], vec![nodes.skinned, 1]]
        );
        assert_eq!(calls(&mut e, 0x0044_59e0), vec![vec![0x7100, 1]]);
        // Nothing worn: both 0, then the head nodes get 0 and the second part 1.
        let mut e = world();
        let this = new_npc(&mut e);
        let nodes = face_nodes(&mut e, &[]);
        started(&mut e);
        e.call(0x0060_62e0, &args![this, 0u32, nodes.root]);
        assert_eq!(
            calls(&mut e, 0x0045_0f90),
            vec![
                vec![nodes.biped, 0],
                vec![nodes.skinned, 0],
                vec![nodes.head, 0],
                vec![nodes.parts[0], 0],
                vec![nodes.parts[1], 1],
            ]
        );
        // Slot 1 worn: the head and first part get 1; slot 10 also worn:
        // the second part gets 0.
        let mut e = world();
        let this = new_npc(&mut e);
        let nodes = face_nodes(&mut e, &[(1, 0x7101), (10, 0x710a)]);
        started(&mut e);
        e.call(0x0060_62e0, &args![this, 0u32, nodes.root]);
        assert_eq!(
            calls(&mut e, 0x0045_0f90)[2..].to_vec(),
            vec![
                vec![nodes.head, 1],
                vec![nodes.parts[0], 1],
                vec![nodes.parts[1], 0],
            ]
        );
        assert_eq!(
            calls(&mut e, 0x0044_59e0),
            vec![vec![0x7101, 1], vec![0x710a, 1]]
        );
    }

    #[test]
    fn face_nodes_need_a_root_and_both_face_gen_children() {
        // No node given: the actor's root is used; no root: nothing happens.
        let mut e = world();
        let this = new_npc(&mut e);
        let nodes = face_nodes(&mut e, &[]);
        returns(&mut e, 0x0043_fcd0, 0);
        started(&mut e);
        e.call(0x0060_62e0, &args![this, 0x1234u32, 0u32]);
        assert_eq!(calls(&mut e, 0x0043_fcd0), vec![vec![0x1234]]);
        assert!(calls(&mut e, FIND_NODE_BY_NAME).is_empty());
        returns(&mut e, 0x0043_fcd0, nodes.root);
        e.call_log = Some(vec![]);
        e.call(0x0060_62e0, &args![this, 0x1234u32, 0u32]);
        assert_eq!(
            calls(&mut e, FIND_NODE_BY_NAME)[0],
            vec![nodes.root, 0x0102_0408]
        );
        assert_eq!(calls(&mut e, 0x0045_0f90).len(), 5);
        // A missing skinned child: stops before the inventory.
        e.register_double(FIND_NODE_BY_NAME, |_, a| ret((a[1] == 0x0102_0408) as u32));
        e.call_log = Some(vec![]);
        e.call(0x0060_62e0, &args![this, 0u32, nodes.root]);
        assert!(calls(&mut e, 0x004b_f220).is_empty());
    }

    #[test]
    fn rebuilding_a_model_stops_when_the_part_table_is_in_use_unless_forced() {
        let mut e = world();
        let this = new_npc(&mut e);
        let actor = object_with(&mut e, 0x0c00_0000, &[(0x100, 0), (0x22c, 1)]);
        let biped = e.mem.alloc(0x400);
        e.mem.set_u32(biped + 0x16c + 3 * 0x10, 0x9999);
        started(&mut e);
        e.call(0x0060_6540, &args![this, actor, biped, 0u32]);
        assert!(calls(&mut e, 0x004a_b250).is_empty());
        assert!(calls(&mut e, 0x004b_f220).is_empty());
        // Forced: the race and sex go to the biped and the rest runs.
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER_SINGLETON, player);
        e.set(this, TESNPC::iActorBaseFlags, 1);
        e.mem.set_u32(this.addr() + COMPONENT_RACE + 4, 0x7ace);
        e.call_log = Some(vec![]);
        e.call(0x0060_6540, &args![this, actor, biped, 1u32]);
        assert_eq!(calls(&mut e, 0x004a_b250), vec![vec![biped, 0x7ace, 1]]);
        // The default worn items are equipped (fn_006047c0 asks the
        // actor's virtual 0x100) when 005f1590 or force say so.
        assert_eq!(calls(&mut e, 0x005f_1590), vec![vec![this.addr(), actor]]);
    }

    #[test]
    fn rebuilding_a_model_notifies_the_process_unless_a_dismembered_part_is_set() {
        let mut e = world();
        let this = new_npc(&mut e);
        let process = object_with(&mut e, 0x0c00_0000, &[(0x468, 0)]);
        let actor = e.mem.alloc(0x100);
        e.mem.set_u32(actor + 0x68, process);
        let biped = e.mem.alloc(0x400);
        returns(&mut e, 0x005d_43c0, 0x8000);
        let extra = e.mem.alloc(0x40);
        e.mem.set_u32(extra + 0x28, 2);
        returns(&mut e, 0x0042_e8c0, extra);
        let flags = block(&mut e, 0x10, &[(0, 0x0100)]);
        // Entry 0: first byte 0, second byte 1 (flag set) blocks the notification.
        e.register_double(0x0044_1420, move |_, a| {
            ret(if a[0] == extra { flags + a[1] * 2 } else { 0 })
        });
        returns(&mut e, 0x0043_fcd0, 0x1);
        started(&mut e);
        e.call(0x0060_6540, &args![this, actor, biped, 0u32]);
        assert!(calls(&mut e, slot(0x0c00_0000, 0x468)).is_empty());
        // No flagged entry: the process is told (virtual 0x468, 1).
        e.mem.set_u8(flags + 1, 0);
        e.call_log = Some(vec![]);
        e.call(0x0060_6540, &args![this, actor, biped, 0u32]);
        assert_eq!(
            calls(&mut e, slot(0x0c00_0000, 0x468)),
            vec![vec![process, 1]]
        );
        // The player's base form being this NPC cancels the default
        // equipment when anything is worn.
        let player = e.mem.alloc(0x100);
        e.mem.set_u32(player + 0x20, this.addr());
        e.set_global(PLAYER_SINGLETON, player);
        returns(&mut e, 0x0043_fcd0, 0);
        returns(&mut e, 0x005f_1590, 1);
        returns(&mut e, 0x004b_f220, 0x5000);
        let entry = e.mem.alloc(0x20);
        returns(&mut e, 0x004c_8c10, entry);
        e.call_log = Some(vec![]);
        e.call(0x0060_6540, &args![this, actor, biped, 0u32]);
        assert_eq!(calls(&mut e, 0x0044_59e0)[0], vec![entry, 1]);
        assert!(calls(&mut e, 0x0093_44a0).is_empty());
    }

    #[test]
    fn part_table_address_is_biped_plus_0x16c() {
        let mut e = world();
        assert_eq!(e.call(0x0060_6800, &args![0x1000u32]).u32(), 0x116c);
    }

    /// The objects `LinearFaceGenHeadLoad` works on.
    struct HeadLoad {
        e: Engine,
        this: Ptr<TESNPC>,
        actor: u32,
        biped: u32,
        root: u32,
        attach: u32,
    }

    fn head_load_world() -> HeadLoad {
        let mut e = world();
        let this = new_npc(&mut e);
        e.set(this, TESNPC::iFormID, 0x0777_0001);
        let root = object_with(&mut e, 0x0c10_0000, &[(0xdc, 0), (0x128, 0), (0x100, 0)]);
        let node = object_with(&mut e, 0x0c11_0000, &[(0xc, root)]);
        let attach = object_with(&mut e, 0x0c12_0000, &[(0xdc, 0)]);
        let actor = object_with(&mut e, 0x0c13_0000, &[(0x1ac, 0), (0x1b0, 0), (0x1e4, 0)]);
        let biped = block(&mut e, 0x40, &[(0, 0x1)]);
        returns(&mut e, 0x0065_1b30, 1);
        returns(&mut e, 0x0043_faf0, 1);
        returns(&mut e, 0x0043_fcd0, node);
        returns(&mut e, 0x004a_b230, attach);
        HeadLoad {
            e,
            this,
            actor,
            biped,
            root,
            attach,
        }
    }

    /// A head node with the virtual slots `LinearFaceGenHeadLoad` calls.
    fn fake_head_node(e: &mut Engine, table: u32, property: u32) -> u32 {
        object_with(
            e,
            table,
            &[
                (0x100, property),
                (0x114, 0),
                (0x11c, 0),
                (0xb4, 0),
                (0x128, 0),
            ],
        )
    }

    #[test]
    fn head_load_does_nothing_without_the_manager_a_biped_or_the_player_first_person() {
        let mut w = head_load_world();
        returns(&mut w.e, 0x0065_1b30, 0);
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        assert!(calls(&mut w.e, 0x0043_fcd0).is_empty());
        assert_eq!(calls(&mut w.e, NI_POINTER_CONSTRUCT).len(), 1);
        assert_eq!(calls(&mut w.e, NI_POINTER_DESTROY).len(), 1);
        // The biped's first word is empty.
        let mut w = head_load_world();
        let empty = w.e.mem.alloc(0x20);
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, empty]);
        assert!(calls(&mut w.e, 0x0043_fcd0).is_empty());
        // The manager is there but 0043faf0 says no.
        let mut w = head_load_world();
        returns(&mut w.e, 0x0043_faf0, 0);
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        assert!(calls(&mut w.e, 0x0043_fcd0).is_empty());
        // The player, with 00950b30 true for the biped.
        let mut w = head_load_world();
        w.e.set_global(PLAYER_SINGLETON, w.actor);
        returns(&mut w.e, 0x0095_0b30, 1);
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        assert_eq!(calls(&mut w.e, 0x0095_0b30), vec![vec![w.actor, w.biped]]);
        assert!(calls(&mut w.e, 0x0043_fcd0).is_empty());
    }

    #[test]
    fn head_load_logs_a_missing_biped_head_node_and_leaves_an_attached_head_alone() {
        let mut w = head_load_world();
        returns(&mut w.e, 0x004a_b230, 0);
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        assert_eq!(
            calls(&mut w.e, LOG_MESSAGE),
            vec![vec![0x0104_a860, 0x0777_0001]]
        );
        assert!(calls(&mut w.e, 0x00a5_9c60).is_empty());
    }

    #[test]
    fn head_load_only_updates_when_the_actor_already_has_the_heads() {
        let mut w = head_load_world();
        // The actor's virtual 0x1ac (for the biped head) already answers.
        let busy = object_with(&mut w.e, 0x0c14_0000, &[(0x1ac, 1), (0x1b0, 0), (0x1e4, 0)]);
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, busy, w.biped]);
        let update = calls(&mut w.e, 0x00a5_9c60);
        assert_eq!(update.len(), 1);
        assert_eq!(update[0][0], w.root);
        assert!(calls(&mut w.e, NI_POINTER_CONSTRUCT).len() == 1);
        assert!(calls(&mut w.e, 0x0061_3c50).is_empty());
    }

    #[test]
    fn head_load_asks_the_race_for_heads_and_attaches_them() {
        let mut w = head_load_world();
        let race = w.e.mem.alloc(0x600);
        w.e.mem.set_u16(race + 0x4f8, 0x2a);
        w.e.mem.set_u32(w.this.addr() + COMPONENT_RACE + 4, race);
        let property = object_with(&mut w.e, 0x0c15_0000, &[(0xd8, 0), (0xd0, 0), (0xb4, 0)]);
        let head_a = fake_head_node(&mut w.e, 0x0c16_0000, property);
        let head_b = fake_head_node(&mut w.e, 0x0c17_0000, 0);
        let (a, b) = (head_a, head_b);
        w.e.register_double(0x0061_3c50, move |e, args| {
            e.mem.set_u32(args[1], a);
            e.mem.set_u32(args[2], b);
            Ret::default()
        });
        returns(&mut w.e, 0x0049_6940, 0);
        returns(&mut w.e, 0x0064_c5a0, 0);
        w.e.register(0x0056_8ad0, |_, _| ret_float(0.0));
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        let this = w.this;
        assert_eq!(w.e.get(this, TESNPC::sLastRaceFaceNum), 0x2a);
        assert_eq!(w.e.mem.u32(this.addr() + 0x1c4), head_a);
        assert_eq!(w.e.mem.u32(this.addr() + 0x1c8), head_b);
        let factory = calls(&mut w.e, 0x0061_3c50);
        assert_eq!(factory.len(), 1);
        assert_eq!(factory[0][0], race);
        assert_eq!(&factory[0][3..], &[this.addr(), 0, 0, 0]);
        // The first head: property hidden (the actor's scale is 0.0), flags,
        // the shared transform and attachment, then its owner.
        assert_eq!(
            calls(&mut w.e, slot(0x0c15_0000, 0xd8)),
            vec![vec![property, 1, 1]]
        );
        assert_eq!(
            calls(&mut w.e, slot(0x0c15_0000, 0xd0)),
            vec![vec![property, 1]]
        );
        assert_eq!(
            calls(&mut w.e, slot(0x0c16_0000, 0x114)),
            vec![vec![head_a, 1]]
        );
        assert_eq!(
            calls(&mut w.e, slot(0x0c16_0000, 0x11c)),
            vec![vec![head_a, 1]]
        );
        assert_eq!(
            calls(&mut w.e, 0x0043_fa80),
            vec![vec![head_a, 0x011a_9448]]
        );
        assert_eq!(
            calls(&mut w.e, slot(0x0c12_0000, 0xdc)),
            vec![vec![w.attach, head_a, 1]]
        );
        assert_eq!(w.e.mem.u32(head_a + 0xe8), w.actor);
        // The second head hangs off the root, flagged by whether the first exists.
        assert_eq!(
            calls(&mut w.e, slot(0x0c17_0000, 0x114)),
            vec![vec![head_b, 0]]
        );
        assert_eq!(
            calls(&mut w.e, slot(0x0c10_0000, 0xdc)),
            vec![vec![w.root, head_b, 1]]
        );
        assert_eq!(w.e.mem.u32(head_b + 0xe8), w.actor);
        assert_eq!(
            calls(&mut w.e, slot(0x0c17_0000, 0x128)),
            vec![vec![head_b, w.root, 1]]
        );
        // The first head's property gets the update call.
        assert_eq!(
            calls(&mut w.e, slot(0x0c15_0000, 0xb4)),
            vec![vec![property, 0.0f32.to_bits(), 1, 1, 1, 1, 0]]
        );
        let update = calls(&mut w.e, 0x00a5_9c60);
        assert_eq!(update.len(), 1);
        assert_eq!(update[0][0], w.root);
    }

    #[test]
    fn head_load_logs_a_race_that_makes_no_head() {
        let mut w = head_load_world();
        let race = w.e.mem.alloc(0x600);
        w.e.mem.set_u32(w.this.addr() + COMPONENT_RACE + 4, race);
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        assert_eq!(
            calls(&mut w.e, LOG_MESSAGE),
            vec![vec![0x0104_a8b8, 0x0777_0001]]
        );
        // Without a race nothing is asked and the same message appears.
        let mut w = head_load_world();
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        assert!(calls(&mut w.e, 0x0061_3c50).is_empty());
        assert_eq!(calls(&mut w.e, LOG_MESSAGE).len(), 1);
    }

    #[test]
    fn head_load_reuses_unshared_heads_and_clones_shared_ones() {
        // The skinned head has two references: the biped head is cloned.
        let mut w = head_load_world();
        let property = object_with(&mut w.e, 0x0c15_0000, &[(0xd8, 0), (0xd0, 0), (0xb4, 0)]);
        let biped_head = fake_head_node(&mut w.e, 0x0c16_0000, property);
        let skinned_head = fake_head_node(&mut w.e, 0x0c17_0000, 0);
        let clone = fake_head_node(&mut w.e, 0x0c18_0000, 0);
        w.e.mem.set_u32(skinned_head + 4, 2);
        w.e.mem.set_u32(w.this.addr() + 0x1c4, biped_head);
        w.e.mem.set_u32(w.this.addr() + 0x1c8, skinned_head);
        w.e.register_double(0x00a5_d2c0, move |_, a| {
            ret(if a[0] == biped_head { clone } else { a[0] })
        });
        returns(&mut w.e, 0x0049_6940, 0);
        w.e.register(0x0056_8ad0, |_, _| ret_float(1.0));
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        let made = calls(&mut w.e, 0x004a_d050);
        assert_eq!(made.len(), 2);
        assert_eq!(made[0][1], 1.0f32.to_bits());
        assert_eq!(calls(&mut w.e, 0x004a_d270).len(), 2);
        // The biped head (shared with the skinned head's two references) is
        // cloned and the clone is attached; the skinned head is cloned too.
        let attached = calls(&mut w.e, slot(0x0c12_0000, 0xdc));
        assert_eq!(attached, vec![vec![w.attach, clone, 1]]);
        // Unshared (one reference): both are used as they are.
        let mut w = head_load_world();
        let biped_head = fake_head_node(&mut w.e, 0x0c16_0000, 0);
        let skinned_head = fake_head_node(&mut w.e, 0x0c17_0000, 0);
        w.e.mem.set_u32(skinned_head + 4, 1);
        w.e.mem.set_u32(w.this.addr() + 0x1c4, biped_head);
        w.e.mem.set_u32(w.this.addr() + 0x1c8, skinned_head);
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        assert!(calls(&mut w.e, 0x004a_d050).is_empty());
        assert_eq!(
            calls(&mut w.e, slot(0x0c12_0000, 0xdc)),
            vec![vec![w.attach, biped_head, 1]]
        );
        assert_eq!(
            calls(&mut w.e, slot(0x0c10_0000, 0xdc)),
            vec![vec![w.root, skinned_head, 1]]
        );
    }

    #[test]
    fn head_load_copies_child_data_and_rebuilds_the_skin_remap() {
        let mut w = head_load_world();
        // The skinned head (one reference) has one child whose data is copied
        // and whose skin instance gets its bone table remapped.
        let cast = object_with(&mut w.e, 0x0c20_0000, &[(0xe4, 0)]);
        let child = object_with(&mut w.e, 0x0c21_0000, &[(0x1c, cast)]);
        let skinned_head = fake_head_node(&mut w.e, 0x0c17_0000, 0);
        w.e.mem.set_u32(skinned_head + 4, 1);
        w.e.mem.set_u32(w.this.addr() + 0x1c8, skinned_head);
        w.e.register_double(0x0043_b480, move |_, a| ret((a[0] == skinned_head) as u32));
        w.e.register_double(0x0043_b4a0, move |_, a| {
            ret(if a[0] == skinned_head && a[1] == 0 {
                child
            } else {
                0
            })
        });
        // The head is the one the NPC points at.
        w.e.register_double(0x0052_aa80, move |_, a| ret((a[1] == skinned_head) as u32));
        // 005495f0 -> data object; 00a5d510 stores a copy in the pointer.
        let copy_value = w.e.mem.alloc(0x20);
        returns(&mut w.e, 0x0054_95f0, 0x4444);
        w.e.register_double(0x00a5_d510, move |e, a| {
            e.mem.set_u32(a[1], copy_value);
            Ret::default()
        });
        // The skin instance: 0043fad0(cast) -> skin, 0043b230(skin) -> shape.
        let skin = w.e.mem.alloc(0x20);
        returns(&mut w.e, 0x0043_fad0, skin);
        returns(&mut w.e, 0x0043_b230, 0x5555);
        // The remapper found on the cast: its virtual 0x94 gives a table whose
        // inner node holds the bone values.
        let table_target = w.e.mem.alloc(0x20);
        let inner = block(&mut w.e, 0x40, &[(0x10, table_target)]);
        let table = block(&mut w.e, 0x20, &[(8, inner)]);
        let remapper = object_with(&mut w.e, 0x0c22_0000, &[(0x94, table)]);
        returns(&mut w.e, 0x00a5_bdd0, remapper);
        let values = block(&mut w.e, 0x20, &[(0, 0x11), (4, 0x22)]);
        returns(&mut w.e, 0x0082_5c00, values);
        returns(&mut w.e, 0x0080_41a0, 2);
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        assert_eq!(
            calls(&mut w.e, 0x004a_dda0),
            vec![vec![skin, 0, 0x11], vec![skin, 1, 0x22]]
        );
        // The copy of the child's data: the virtual 0xe4 of the child's cast
        // gets the stored pointer, and the skin gets the copy of its shape.
        assert_eq!(
            calls(&mut w.e, slot(0x0c20_0000, 0xe4)),
            vec![vec![cast, copy_value]]
        );
        assert_eq!(calls(&mut w.e, 0x004a_ddc0), vec![vec![skin, copy_value]]);
        assert!(calls(&mut w.e, LOG_MESSAGE)
            .iter()
            .all(|c| c[0] != 0x0104_a908));
    }

    #[test]
    fn head_load_logs_a_skin_it_cannot_remap() {
        let mut w = head_load_world();
        let cast = object_with(&mut w.e, 0x0c20_0000, &[(0xe4, 0)]);
        let child = object_with(&mut w.e, 0x0c21_0000, &[(0x1c, cast)]);
        let skinned_head = fake_head_node(&mut w.e, 0x0c17_0000, 0);
        w.e.mem.set_u32(skinned_head + 4, 1);
        w.e.mem.set_u32(w.this.addr() + 0x1c8, skinned_head);
        w.e.register_double(0x0043_b480, move |_, a| ret((a[0] == skinned_head) as u32));
        w.e.register_double(0x0043_b4a0, move |_, _| ret(child));
        returns(&mut w.e, 0x0043_fad0, 0x4000);
        returns(&mut w.e, 0x0043_b230, 0x5555);
        // No remapper (00a5bdd0 returns 0): the NPC names itself in the log.
        returns(&mut w.e, 0x0c24_1000, 0x6666);
        put_object_vtable(
            &mut w.e,
            w.this.addr(),
            0x0c24_0000,
            &[(VSLOT_GET_FORM_NAME, 0x0c24_1000)],
        );
        started(&mut w.e);
        w.e.call(0x0060_6820, &args![w.this, w.actor, w.biped]);
        assert_eq!(
            calls(&mut w.e, LOG_MESSAGE)[0],
            vec![0x0104_a908, 0x6666, 0x0777_0001]
        );
        assert!(calls(&mut w.e, 0x004a_dda0).is_empty());
    }

    #[test]
    fn small_head_accessors() {
        let mut e = world();
        // 006072c0: the skin data behind the NiPointer of the inner node.
        let target = e.mem.alloc(0x20);
        let inner = block(&mut e, 0x40, &[(0x10, target)]);
        let this = block(&mut e, 0x20, &[(8, inner)]);
        returns(&mut e, 0x0043_fad0, 0x7777);
        assert_eq!(e.call(0x0060_72c0, &args![this]).u32(), 0x7777);
        let empty = block(&mut e, 0x20, &[]);
        assert_eq!(e.call(0x0060_72c0, &args![empty]).u32(), 0);
        let hollow_inner = block(&mut e, 0x40, &[]);
        let hollow = block(&mut e, 0x20, &[(8, hollow_inner)]);
        assert_eq!(e.call(0x0060_72c0, &args![hollow]).u32(), 0);
        // 00607340 / 00607310: a global key and the extra-data lookup.
        e.set_global(0x011d_5b50, 0x5150u32);
        assert_eq!(e.call(0x0060_7340, &args![]).u32(), 0x5150);
        returns(&mut e, 0x00a5_bdd0, 0x9090);
        started(&mut e);
        assert_eq!(e.call(0x0060_7310, &args![0x1111u32]).u32(), 0x9090);
        assert_eq!(calls(&mut e, 0x00a5_bdd0), vec![vec![0x1111, 0x5150]]);
        assert_eq!(e.call(0x0060_7310, &args![0u32]).u32(), 0);
        // 00607350: the race's face number.
        let race = e.mem.alloc(0x600);
        e.mem.set_u16(race + 0x4f8, 0x1234);
        assert_eq!(e.call(0x0060_7350, &args![race]).u16(), 0x1234);
    }

    #[test]
    fn init_head_clears_the_heads_and_asks_the_race_when_they_stay_empty() {
        let mut e = world();
        let this = new_npc(&mut e);
        let race = e.mem.alloc(0x600);
        e.mem.set_u16(race + 0x4f8, 0x31);
        e.mem.set_u32(this.addr() + COMPONENT_RACE + 4, race);
        e.mem.set_u32(this.addr() + 0x1c4, 0x1111);
        e.mem.set_u32(this.addr() + 0x1c8, 0x2222);
        started(&mut e);
        e.call(0x0060_7370, &args![this, 0xaau32, 0xbbu32]);
        assert_eq!(e.mem.u32(this.addr() + 0x1c4), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x1c8), 0);
        assert_eq!(
            calls(&mut e, 0x0061_3c50),
            vec![vec![race, 0xaa, 0xbb, this.addr(), 1, 0, 0]]
        );
        assert_eq!(e.get(this, TESNPC::sLastRaceFaceNum), 0x31);
        // No race: only the clearing.
        let mut e = world();
        let this = new_npc(&mut e);
        started(&mut e);
        e.call(0x0060_7370, &args![this, 0xaau32, 0xbbu32]);
        assert!(calls(&mut e, 0x0061_3c50).is_empty());
        assert_eq!(calls(&mut e, NI_POINTER_ASSIGN).len(), 2);
    }

    /// The head world plus a property object for `fn_00607420`.
    #[test]
    fn attach_heads_hangs_both_nodes_and_stores_the_actors_head_values() {
        let mut w = head_load_world();
        let property = object_with(&mut w.e, 0x0c15_0000, &[(0xd8, 0), (0xd0, 0), (0xb4, 0)]);
        let head_a = fake_head_node(&mut w.e, 0x0c16_0000, property);
        let head_b = fake_head_node(&mut w.e, 0x0c17_0000, 0);
        let actor = object_with(&mut w.e, 0x0c19_0000, &[(0x1e4, 0), (0x100, 1)]);
        let table = w.e.mem.alloc(0x400);
        w.e.mem.set_u32(actor + 0xac, table);
        returns(&mut w.e, 0x0049_6940, 0);
        w.e.register(0x0056_8ad0, |_, _| ret_float(2.0));
        // Each of the four helpers writes its two outputs.
        w.e.register(0x0064_9f00, |e, a| {
            e.mem.set_f32(a[0], 1.0);
            e.mem.set_f32(a[1], 2.0);
            Ret::default()
        });
        w.e.register(0x0064_9f70, |e, a| {
            e.mem.set_f32(a[0], 3.0);
            e.mem.set_f32(a[1], 4.0);
            Ret::default()
        });
        w.e.register(0x0064_9fe0, |e, a| {
            e.mem.set_f32(a[0], 5.0);
            e.mem.set_f32(a[1], 6.0);
            Ret::default()
        });
        w.e.register(0x0064_a070, |e, a| {
            e.mem.set_f32(a[0], 7.0);
            e.mem.set_f32(a[1], 8.0);
            Ret::default()
        });
        started(&mut w.e);
        w.e.call(0x0060_7420, &args![w.this, actor, w.biped, head_a, head_b]);
        let this = w.this;
        assert_eq!(w.e.mem.u32(this.addr() + 0x1c4), head_a);
        assert_eq!(w.e.mem.u32(this.addr() + 0x1c8), head_b);
        // The head values: 4.0 (second helper's second output) at +0x1a8,
        // 2.0 into record 0, then 6.0 and 8.0 to 00c748d0.
        assert_eq!(w.e.mem.f32(table + 0x1a8), 4.0);
        assert_eq!(w.e.mem.f32(table + 0xf8), 2.0);
        assert_eq!(
            calls(&mut w.e, 0x00c7_48d0),
            vec![vec![table, 0, 6.0f32.to_bits(), 8.0f32.to_bits()]]
        );
        // The scale (2.0) is above 0.0: the property keeps its flags.
        assert!(calls(&mut w.e, slot(0x0c15_0000, 0xd8)).is_empty());
        assert_eq!(
            calls(&mut w.e, slot(0x0c15_0000, 0xb4)),
            vec![vec![property, 0.0f32.to_bits(), 1, 1, 1, 1, 0]]
        );
        assert_eq!(
            calls(&mut w.e, slot(0x0c12_0000, 0xdc)),
            vec![vec![w.attach, head_a, 1]]
        );
        assert_eq!(
            calls(&mut w.e, slot(0x0c10_0000, 0xdc)),
            vec![vec![w.root, head_b, 1]]
        );
        // The root is updated and 006062e0 asks for the root again at the end.
        assert_eq!(calls(&mut w.e, 0x00a5_9c60)[0][0], w.root);
        assert!(calls(&mut w.e, 0x0043_fcd0).len() >= 2);
        // Without the kind of actor that has head values nothing is stored.
        let mut w = head_load_world();
        let actor = object_with(&mut w.e, 0x0c19_0000, &[(0x1e4, 0), (0x100, 0)]);
        let head_a = fake_head_node(&mut w.e, 0x0c16_0000, 0);
        started(&mut w.e);
        w.e.call(0x0060_7420, &args![w.this, actor, w.biped, head_a, 0u32]);
        assert!(calls(&mut w.e, 0x0064_9f00).is_empty());
        // A missing head node of the biped is logged and nothing is attached.
        let mut w = head_load_world();
        returns(&mut w.e, 0x004a_b230, 0);
        started(&mut w.e);
        w.e.call(0x0060_7420, &args![w.this, w.actor, w.biped, 0u32, 0u32]);
        assert_eq!(
            calls(&mut w.e, LOG_MESSAGE),
            vec![vec![0x0104_a860, 0x0777_0001]]
        );
        assert!(calls(&mut w.e, 0x00a5_9c60).is_empty());
        assert!(!calls(&mut w.e, 0x0043_fcd0).is_empty());
    }

    #[test]
    fn head_value_setters_store_floats_in_the_exes_places() {
        let mut e = world();
        let target = e.mem.alloc(0x400);
        e.call(0x0060_7810, &args![target, 2u32, 1.5f32]);
        assert_eq!(e.mem.f32(target + 0xf8 + 0xa0), 1.5);
        e.call(0x0060_7830, &args![target, 2.5f32]);
        assert_eq!(e.mem.f32(target + 0x1a8), 2.5);
    }

    #[test]
    fn actor_value_getter_adds_the_offset_unless_the_npc_is_auto_calculated() {
        let mut e = world();
        let npc = new_npc(&mut e);
        e.mem.set_u8(npc.addr() + 0x114 + 3, 40);
        e.mem.set_u8(npc.addr() + 0x122 + 3, 7);
        e.register(0x0047_f060, |_, a| ret((a[0] == 5) as u32));
        e.register(0x0066_ec80, |_, a| {
            assert_eq!((a[0], a[1]), (2, 5));
            ret(3)
        });
        put_object_vtable(&mut e, npc.addr(), 0x0c00_0000, &[(0x144, 0x0c00_1144)]);
        returns(&mut e, 0x0c00_1144, 0);
        assert_eq!(
            e.call(0x0060_7850, &args![npc.addr() + 0x100, 5u32]).u32(),
            47
        );
        returns(&mut e, 0x0c00_1144, 1);
        assert_eq!(
            e.call(0x0060_7850, &args![npc.addr() + 0x100, 5u32]).u32(),
            40
        );
        // An index the exe's test rejects goes to 005f0fb0.
        returns(&mut e, 0x005f_0fb0, 99);
        started(&mut e);
        assert_eq!(
            e.call(0x0060_7850, &args![npc.addr() + 0x100, 9u32]).u32(),
            99
        );
        assert_eq!(
            calls(&mut e, 0x005f_0fb0),
            vec![vec![npc.addr() + 0x100, 9]]
        );
        // A negative skill index sign-extends.
        e.register(0x0066_ec80, |_, _| ret(0xff));
        e.mem.set_u8(npc.addr() + 0x114 - 1, 11);
        e.mem.set_u8(npc.addr() + 0x122 - 1, 4);
        returns(&mut e, 0x0c00_1144, 0);
        assert_eq!(
            e.call(0x0060_7850, &args![npc.addr() + 0x100, 5u32]).u32(),
            15
        );
    }

    #[test]
    fn actor_value_setter_writes_the_skill_byte_or_defers() {
        let mut e = world();
        let npc = new_npc(&mut e);
        e.register(0x0047_f060, |_, a| ret((a[0] == 5) as u32));
        e.register(0x0066_ec80, |_, _| ret(3));
        put_object_vtable(&mut e, npc.addr(), 0x0c00_0000, &[(0x48, 0x0c00_1048)]);
        returns(&mut e, 0x0c00_1048, 0);
        started(&mut e);
        e.call(0x0060_78e0, &args![npc, 5u32, 0x1234_5677u32]);
        assert_eq!(e.mem.u8(npc.addr() + 0x114 + 3), 0x77);
        assert_eq!(calls(&mut e, 0x0c00_1048), vec![vec![npc.addr(), 0x200]]);
        started(&mut e);
        e.call(0x0060_78e0, &args![npc, 9u32, 0x31u32]);
        assert_eq!(calls(&mut e, 0x005f_12d0), vec![vec![npc.addr(), 9, 0x31]]);
        assert!(calls(&mut e, 0x0c00_1048).is_empty());
    }

    #[test]
    fn combat_style_accessors() {
        let mut e = world();
        let npc = new_npc(&mut e);
        e.call(0x0060_7970, &args![npc, 0x3333u32]);
        assert_eq!(e.mem.u32(npc.addr() + 0x1d4), 0x3333);
        assert_eq!(e.call(0x0060_7950, &args![npc]).u32(), 0x3333);
    }

    #[test]
    fn save_size_and_save_read_write_the_npc_data_block() {
        let mut e = world();
        let npc = new_npc(&mut e);
        returns(&mut e, 0x005f_16f0, 0x30);
        assert_eq!(e.call(0x0060_8da0, &args![npc, 0x200u32]).u16(), 0x3e);
        assert_eq!(e.call(0x0060_8da0, &args![npc, 0u32]).u16(), 0x30);
        returns(&mut e, 0x005f_16f0, 0xfffa);
        assert_eq!(e.call(0x0060_8da0, &args![npc, 0x200u32]).u16(), 8);
        started(&mut e);
        e.call(0x0060_8e00, &args![npc, 0x200u32]);
        assert_eq!(calls(&mut e, 0x005f_18c0), vec![vec![npc.addr(), 0x200]]);
        assert_eq!(
            calls(&mut e, 0x0048_4ce0),
            vec![vec![npc.addr(), npc.addr() + 0x114, 0xe]]
        );
        started(&mut e);
        e.call(0x0060_8e00, &args![npc, 0x1u32]);
        assert!(calls(&mut e, 0x0048_4ce0).is_empty());
        started(&mut e);
        e.call(0x0060_8e80, &args![npc, 0x201u32, 0x77u32]);
        assert_eq!(
            calls(&mut e, 0x005f_1b30),
            vec![vec![npc.addr(), 0x201, 0x77]]
        );
        assert_eq!(
            calls(&mut e, 0x0048_4d00),
            vec![vec![npc.addr(), npc.addr() + 0x114, 0xe]]
        );
        started(&mut e);
        e.call(0x0060_8e80, &args![npc, 0x1u32, 0u32]);
        assert!(calls(&mut e, 0x0048_4d00).is_empty());
    }

    #[test]
    fn four_is_the_kind_the_first_helper_tests_for() {
        let mut e = world();
        returns(&mut e, 0x004f_8960, 4);
        assert!(e.call(0x0060_8d80, &args![0x1000u32]).bool());
        returns(&mut e, 0x004f_8960, 5);
        assert!(!e.call(0x0060_8d80, &args![0x1000u32]).bool());
    }

    #[test]
    fn file_check_needs_an_open_file_the_offset_and_a_matching_record() {
        let mut e = world();
        let npc = new_npc(&mut e);
        e.set(npc, TESNPC::iFileOffset, 0x40);
        e.set(npc, TESNPC::iFormID, 0x1234);
        e.map(0x0118_7000, 0x1000);
        e.set_global(0x0118_71f8, 0x2au8);
        let file = e.mem.alloc(0x40);
        returns(&mut e, 0x0047_0c70, 1);
        returns(&mut e, 0x0047_23a0, 1);
        returns(&mut e, FILE_GET_FORM_TYPE, 0x2a);
        returns(&mut e, 0x008d_8ac0, 0x1234);
        started(&mut e);
        assert!(e.call(0x0060_9bf0, &args![npc, file]).bool());
        assert_eq!(calls(&mut e, 0x0047_0c70), vec![vec![file, 0, 0]]);
        assert_eq!(calls(&mut e, 0x0047_23a0), vec![vec![file, 0x40]]);
        // Each failure.
        assert!(!e.call(0x0060_9bf0, &args![npc, 0u32]).bool());
        returns(&mut e, 0x008d_8ac0, 0x9999);
        assert!(!e.call(0x0060_9bf0, &args![npc, file]).bool());
        returns(&mut e, 0x008d_8ac0, 0x1234);
        returns(&mut e, FILE_GET_FORM_TYPE, 0x2b);
        assert!(!e.call(0x0060_9bf0, &args![npc, file]).bool());
        returns(&mut e, FILE_GET_FORM_TYPE, 0x2a);
        returns(&mut e, 0x0047_23a0, 0);
        assert!(!e.call(0x0060_9bf0, &args![npc, file]).bool());
        returns(&mut e, 0x0047_23a0, 1);
        returns(&mut e, 0x0047_0c70, 0);
        assert!(!e.call(0x0060_9bf0, &args![npc, file]).bool());
        returns(&mut e, 0x0047_0c70, 1);
        e.set(npc, TESNPC::iFileOffset, 0);
        assert!(!e.call(0x0060_9bf0, &args![npc, file]).bool());
    }

    #[test]
    fn old_face_data_size_counts_four_bytes_per_matrix_element_plus_21() {
        let mut e = world();
        let npc = new_npc(&mut e);
        returns(&mut e, 0x0096_11e0, 2);
        returns(&mut e, 0x0044_1110, 3);
        assert_eq!(
            e.call(0x0060_9c70, &args![npc, 0u32]).u16(),
            4 * 2 * 3 * 4 + 21
        );
        // The sum is a 16-bit value.
        returns(&mut e, 0x0096_11e0, 0x4000);
        returns(&mut e, 0x0044_1110, 2);
        assert_eq!(e.call(0x0060_9c70, &args![npc, 0u32]).u16(), 21);
    }

    #[test]
    fn old_face_data_is_written_element_by_element_then_ids_and_the_sex_byte() {
        let mut e = world();
        let npc = new_npc(&mut e);
        e.set(npc, TESNPC::iActorBaseFlags, 1);
        e.mem.set_u32(npc.addr() + COMPONENT_RACE + 4, 0xa001);
        e.set(npc, TESNPC::pHair, Ptr::new(0xa002));
        e.set(npc, TESNPC::fHairLength, 0.5);
        e.set(npc, TESNPC::iHairColor, 0x0102_0304);
        returns(&mut e, 0x0096_11e0, 1);
        returns(&mut e, 0x0044_1110, 1);
        let cells = block(&mut e, 0x20, &[(0, 0x4040_0000)]);
        returns(&mut e, MATRIX_ITERATOR_AT, 0x1111);
        returns(&mut e, ITERATOR_ELEMENT, cells);
        // Form ids come from 0084e3a0: the id word of the form.
        e.register(GET_FORM_ID, |_, a| ret(a[0] + 1));
        let written = Rc::new(RefCell::new(vec![]));
        for (address, tag) in [(0x0048_4ce0u32, 'D'), (0x0048_4d20u32, 'I')] {
            let sink = written.clone();
            e.register_double(address, move |e, a| {
                sink.borrow_mut().push((tag, e.mem.bytes(a[1], a[2])));
                Ret::default()
            });
        }
        e.call(0x0060_9d60, &args![npc, 0u32]);
        let words = |v: u32| v.to_le_bytes().to_vec();
        let mut expected = vec![('D', words(0x4040_0000)); 4];
        expected.extend([
            ('I', words(0xa002)),
            ('I', words(0xa003)),
            ('I', words(0)),
            ('D', words(0.5f32.to_bits())),
            ('D', words(0x0102_0304)),
            ('D', vec![1]),
        ]);
        assert_eq!(*written.borrow(), expected);
        // No race, hair or eyes: zero ids; a male NPC: sex byte 0.
        let mut e = world();
        let npc = new_npc(&mut e);
        returns(&mut e, 0x0096_11e0, 0);
        let written = Rc::new(RefCell::new(vec![]));
        let sink = written.clone();
        e.register_double(0x0048_4d20, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        let sex = Rc::new(RefCell::new(vec![]));
        let sink = sex.clone();
        e.register_double(0x0048_4ce0, move |e, a| {
            if a[2] == 1 {
                sink.borrow_mut().push(e.mem.u8(a[1]));
            }
            Ret::default()
        });
        e.call(0x0060_9d60, &args![npc, 0u32]);
        assert_eq!(*written.borrow(), vec![0, 0, 0]);
        assert_eq!(*sex.borrow(), vec![0]);
    }

    /// Doubles for the change-flag test of the save/load code:
    /// `0042ce30` and `00428110` return a marker (1 or 2) and
    /// `004280f0(marker, mask)` says whether `mask` is in the flags of that
    /// marker (`loaded` for the first, `saved` for the second).
    fn set_change_flags(e: &mut Engine, loaded: u32, saved: u32) {
        returns(e, 0x0042_ce30, 1);
        returns(e, 0x0042_8110, 2);
        e.register_double(0x0042_80f0, move |_, a| {
            let flags = if a[0] == 1 { loaded } else { saved };
            ret((flags & a[1] != 0) as u32)
        });
    }

    /// The matrix doubles used by the save/load tests: the iterator for
    /// row `x` of a matrix is the matrix address plus `x * 8`, and column
    /// `y` of that is four bytes on, so the NPC's own memory is the storage.
    fn matrix_storage(e: &mut Engine, width: u32, height: u32) {
        returns(e, 0x0096_11e0, width);
        returns(e, 0x0044_1110, height);
        e.register(MATRIX_ITERATOR_AT, |_, a| ret(a[0] + a[2] * 8));
        e.register(ITERATOR_ELEMENT, |_, a| ret(a[0] + a[1] * 4));
    }

    /// What a save-buffer double was asked to write, in order.
    type Written = Rc<RefCell<Vec<(char, Vec<u8>)>>>;

    fn save_recorder(e: &mut Engine) -> Written {
        let written: Written = Rc::new(RefCell::new(vec![]));
        let sink = written.clone();
        e.register_double(0x0086_5e50, move |e, a| {
            sink.borrow_mut().push(('B', e.mem.bytes(a[1], a[2])));
            Ret::default()
        });
        let sink = written.clone();
        e.register_double(0x0086_5df0, move |_, a| {
            sink.borrow_mut().push(('F', a[1].to_le_bytes().to_vec()));
            Ret::default()
        });
        returns(e, 0x0086_5f20, 0x7007);
        let sink = written.clone();
        e.register_double(0x0086_5ff0, move |_, a| {
            let mut bytes = a[1].to_le_bytes().to_vec();
            bytes.extend(a[2].to_le_bytes());
            sink.borrow_mut().push(('S', bytes));
            Ret::default()
        });
        written
    }

    #[test]
    fn save_writes_the_blocks_the_change_flags_ask_for() {
        let mut e = world();
        let npc = new_npc(&mut e);
        e.set(npc, TESNPC::iActorBaseFlags, 1);
        let data: Vec<u8> = (1..=0x1c).collect();
        e.mem.write(npc.addr() + 0x114, &data);
        e.set(npc, TESNPC::pCl, Ptr::new(0xc1a5));
        e.mem.set_u32(npc.addr() + COMPONENT_RACE + 4, 0xa001);
        e.set(npc, TESNPC::pOriginalRace, Ptr::new(0xa002));
        e.set(npc, TESNPC::pHair, Ptr::new(0xa003));
        e.set(npc, TESNPC::pEyeColor, Ptr::new(0xa004));
        e.set(npc, TESNPC::fHairLength, 0.25);
        e.set(npc, TESNPC::iHairColor, 0x0a0b_0c0d);
        // Head parts: 0xb001, none, 0xb003.
        let third = block(&mut e, 0x10, &[(0, 0xb003), (4, 0)]);
        let second = block(&mut e, 0x10, &[(0, 0), (4, third)]);
        e.mem.set_u32(npc.addr() + 0x1dc, 0xb001);
        e.mem.set_u32(npc.addr() + 0x1e0, second);
        matrix_storage(&mut e, 1, 2);
        for k in 0..4u32 {
            // Each matrix has 1 x 2 elements: values 10.0, 11.0, ... in order.
            let matrix = npc.addr() + 0x134 + (k / 2) * 0x40 + (k % 2) * 0x20;
            e.mem.set_f32(matrix, 10.0 + 2.0 * k as f32);
            e.mem.set_f32(matrix + 4, 11.0 + 2.0 * k as f32);
        }
        set_change_flags(&mut e, 0, 0x200 | 0x400 | 0x200_0000 | 0x800 | 0x100_0000);
        let written = save_recorder(&mut e);
        let buffer = e.mem.alloc(0x40);
        started(&mut e);
        e.call(0x0060_8f00, &args![npc, buffer]);
        assert_eq!(calls(&mut e, 0x005f_1f30), vec![vec![npc.addr(), buffer]]);
        let word = |v: u32| v.to_le_bytes().to_vec();
        let mut expected = vec![
            ('B', data),
            ('F', word(0xc1a5)),
            ('F', word(0xa001)),
            ('F', word(0xa002)),
            ('B', vec![0]),
        ];
        for k in 0..8u32 {
            expected.push(('B', word((10.0 + k as f32).to_bits())));
        }
        expected.extend([
            ('F', word(0xa003)),
            ('F', word(0xa004)),
            ('B', word(0.25f32.to_bits())),
            ('B', word(0x0a0b_0c0d)),
            ('F', word(0xb001)),
            ('F', word(0xb003)),
            ('S', [2u32.to_le_bytes(), 0x7007u32.to_le_bytes()].concat()),
            ('B', vec![1]),
        ]);
        assert_eq!(*written.borrow(), expected);
        // The coordinate in use is the alternate one when it is set.
        let alternate = e.mem.alloc(0x80);
        e.set(npc, TESNPC::pAlternateFaceOffsetCoord, Ptr::new(alternate));
        written.borrow_mut().clear();
        started(&mut e);
        e.call(0x0060_8f00, &args![npc, buffer]);
        assert_eq!(written.borrow()[4], ('B', vec![1]));
        assert_eq!(calls(&mut e, 0x0096_11e0)[0], vec![alternate]);
        // No flags: only the base form.
        set_change_flags(&mut e, 0, 0);
        written.borrow_mut().clear();
        e.call(0x0060_8f00, &args![npc, buffer]);
        assert!(written.borrow().is_empty());
    }

    /// What `00864980` should hand back, in order, and what it was asked.
    struct SaveLoadWorld {
        e: Engine,
        npc: Ptr<TESNPC>,
        buffer: u32,
        bytes: Rc<RefCell<std::collections::VecDeque<Vec<u8>>>>,
        ids: Rc<RefCell<std::collections::VecDeque<u32>>>,
    }

    fn save_load_world(flags: u32, version: u32) -> SaveLoadWorld {
        let mut e = world();
        let npc = new_npc(&mut e);
        e.set(npc, TESNPC::iFormID, 0x0777_0001);
        let buffer = object_with(&mut e, 0x0c30_0000, &[(0, version)]);
        set_change_flags(&mut e, 0, flags);
        matrix_storage(&mut e, 1, 2);
        let bytes: Rc<RefCell<std::collections::VecDeque<Vec<u8>>>> =
            Rc::new(RefCell::new(Default::default()));
        let queue = bytes.clone();
        e.register_double(0x0086_4980, move |e, a| {
            let next = queue.borrow_mut().pop_front().expect("a queued load");
            assert!(next.len() as u32 <= a[2], "{} > {}", next.len(), a[2]);
            e.mem.write(a[1], &next);
            Ret::default()
        });
        let ids: Rc<RefCell<std::collections::VecDeque<u32>>> =
            Rc::new(RefCell::new(Default::default()));
        let queue = ids.clone();
        e.register_double(0x0086_48a0, move |_, _| {
            ret(queue.borrow_mut().pop_front().expect("a queued form id"))
        });
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        SaveLoadWorld {
            e,
            npc,
            buffer,
            bytes,
            ids,
        }
    }

    #[test]
    fn load_reads_npc_data_class_and_the_race_with_its_follow_on_changes() {
        // Data and class only; the load flag 0042ce90 adds the 0x1b mask.
        let mut w = save_load_world(0x200 | 0x400, 0);
        let data: Vec<u8> = (1..=0x1c).collect();
        w.bytes.borrow_mut().push_back(data.clone());
        w.ids.borrow_mut().push_back(0x2001);
        returns(&mut w.e, 0x0042_ce90, 1);
        started(&mut w.e);
        w.e.call(0x0060_9220, &args![w.npc, w.buffer]);
        assert_eq!(w.e.mem.bytes(w.npc.addr() + 0x114, 0x1c), data);
        assert_eq!(w.e.get(w.npc, TESNPC::pCl), Ptr::new(0x2001));
        assert_eq!(
            calls(&mut w.e, 0x005f_1fd0),
            vec![vec![w.npc.addr(), w.buffer]]
        );
        assert_eq!(calls(&mut w.e, 0x005f_20a0), vec![vec![w.npc.addr(), 0x1b]]);
        assert_eq!(calls(&mut w.e, 0x005d_d560).len(), 1);
        // A new race: set through 006ecd40; a height equal to the old race's
        // follows the new race; the original race is read too.
        let mut w = save_load_world(0x200_0000, 0);
        let old_race = w.e.mem.alloc(0x600);
        let new_race = w.e.mem.alloc(0x600);
        w.e.mem.set_f32(old_race + 0x60, 1.0);
        w.e.mem.set_f32(new_race + 0x60, 2.0);
        w.e.mem.set_u32(w.npc.addr() + COMPONENT_RACE + 4, old_race);
        w.e.set(w.npc, TESNPC::fHeight, 1.0);
        w.ids.borrow_mut().extend([new_race, 0x3003]);
        started(&mut w.e);
        w.e.call(0x0060_9220, &args![w.npc, w.buffer]);
        assert_eq!(w.e.mem.u32(w.npc.addr() + COMPONENT_RACE + 4), new_race);
        assert_eq!(w.e.get(w.npc, TESNPC::fHeight), 2.0);
        assert_eq!(w.e.get(w.npc, TESNPC::pOriginalRace), Ptr::new(0x3003));
        assert_eq!(calls(&mut w.e, 0x005f_20a0), vec![vec![w.npc.addr(), 0x1b]]);
        // The same race and a height of its own: nothing changes.
        let mut w = save_load_world(0x200_0000, 0);
        let race = w.e.mem.alloc(0x600);
        w.e.mem.set_f32(race + 0x60, 1.0);
        w.e.mem.set_u32(w.npc.addr() + COMPONENT_RACE + 4, race);
        w.e.set(w.npc, TESNPC::fHeight, 1.5);
        w.ids.borrow_mut().extend([race, 0]);
        started(&mut w.e);
        w.e.call(0x0060_9220, &args![w.npc, w.buffer]);
        assert_eq!(w.e.get(w.npc, TESNPC::fHeight), 1.5);
        assert!(calls(&mut w.e, 0x005f_20a0).is_empty());
        assert!(calls(&mut w.e, 0x006e_cd40).is_empty());
    }

    #[test]
    fn load_applies_the_face_and_reports_what_changed() {
        // A new alternate coordinate with one changed element.
        let mut w = save_load_world(0x800, 0xd);
        let mut queue = w.bytes.borrow_mut();
        queue.push_back(vec![1]);
        for k in 0..8u32 {
            queue.push_back(
                (if k == 3 { 5.0f32 } else { 0.0 })
                    .to_bits()
                    .to_le_bytes()
                    .to_vec(),
            );
        }
        queue.push_back(0.5f32.to_bits().to_le_bytes().to_vec());
        queue.push_back(0x0102_0304u32.to_le_bytes().to_vec());
        drop(queue);
        w.ids.borrow_mut().extend([0x4001, 0x4002]);
        w.e.set(w.npc, TESNPC::pHair, Ptr::new(0x4001));
        w.e.set(w.npc, TESNPC::pEyeColor, Ptr::new(0x4002));
        w.e.set(w.npc, TESNPC::fHairLength, 0.5);
        w.e.set(w.npc, TESNPC::iHairColor, 0x0102_0304);
        started(&mut w.e);
        w.e.call(0x0060_9220, &args![w.npc, w.buffer]);
        let alternate = w.e.get(w.npc, TESNPC::pAlternateFaceOffsetCoord);
        assert!(!alternate.is_null());
        assert_eq!(calls(&mut w.e, 0x0065_21f0), vec![vec![alternate.addr()]]);
        assert_eq!(
            calls(&mut w.e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![alternate.addr(), 0x20, 4, 0x0044_9610, 0x0044_9680]]
        );
        // Element 3 is the second element of the second matrix.
        assert_eq!(w.e.mem.f32(alternate.addr() + 0x20 + 4), 5.0);
        assert_eq!(calls(&mut w.e, 0x005d_d560), vec![vec![w.npc.addr()]]);
        assert_eq!(calls(&mut w.e, 0x005f_20a0), vec![vec![w.npc.addr(), 8]]);
        assert_eq!(w.e.get(w.npc, TESNPC::pHair), Ptr::new(0x4001));
        // The same face again changes nothing.
        let mut w = save_load_world(0x800, 0xd);
        let mut queue = w.bytes.borrow_mut();
        queue.push_back(vec![0]);
        for _ in 0..8 {
            queue.push_back(vec![0, 0, 0, 0]);
        }
        queue.push_back(0.5f32.to_bits().to_le_bytes().to_vec());
        queue.push_back(0x0102_0304u32.to_le_bytes().to_vec());
        drop(queue);
        w.ids.borrow_mut().extend([0x4001, 0x4002]);
        w.e.set(w.npc, TESNPC::pHair, Ptr::new(0x4001));
        w.e.set(w.npc, TESNPC::pEyeColor, Ptr::new(0x4002));
        w.e.set(w.npc, TESNPC::fHairLength, 0.5);
        w.e.set(w.npc, TESNPC::iHairColor, 0x0102_0304);
        started(&mut w.e);
        w.e.call(0x0060_9220, &args![w.npc, w.buffer]);
        assert!(calls(&mut w.e, 0x005f_20a0).is_empty());
        assert!(calls(&mut w.e, 0x0065_21f0).is_empty());
        // A different hair colour sets the face bit.
        let mut w = save_load_world(0x800, 0xd);
        let mut queue = w.bytes.borrow_mut();
        queue.push_back(vec![0]);
        for _ in 0..8 {
            queue.push_back(vec![0, 0, 0, 0]);
        }
        queue.push_back(0.5f32.to_bits().to_le_bytes().to_vec());
        queue.push_back(0x0102_0305u32.to_le_bytes().to_vec());
        drop(queue);
        w.ids.borrow_mut().extend([0x4001, 0x4002]);
        w.e.set(w.npc, TESNPC::pHair, Ptr::new(0x4001));
        w.e.set(w.npc, TESNPC::pEyeColor, Ptr::new(0x4002));
        w.e.set(w.npc, TESNPC::fHairLength, 0.5);
        w.e.set(w.npc, TESNPC::iHairColor, 0x0102_0304);
        started(&mut w.e);
        w.e.call(0x0060_9220, &args![w.npc, w.buffer]);
        assert_eq!(calls(&mut w.e, 0x005f_20a0), vec![vec![w.npc.addr(), 8]]);
    }

    #[test]
    fn load_rebuilds_the_head_part_list_for_newer_saves() {
        // The list held part 0xb001 and the save has the same one: no change.
        for (loaded, differs) in [(0xb001u32, false), (0xb002, true)] {
            let mut w = save_load_world(0x800, 0xe);
            let mut queue = w.bytes.borrow_mut();
            queue.push_back(vec![0]);
            for _ in 0..8 {
                queue.push_back(vec![0, 0, 0, 0]);
            }
            queue.push_back(vec![0, 0, 0, 0]);
            queue.push_back(vec![0, 0, 0, 0]);
            drop(queue);
            // hair and eyes (null on both sides), then the part id.
            w.ids.borrow_mut().extend([0, 0, loaded]);
            let list = w.npc.addr() + 0x1dc;
            w.e.mem.set_u32(list, 0xb001);
            returns(&mut w.e, 0x0086_4a60, 1);
            // The array and list doubles: a Rust vector stands in for the
            // array; the list in memory is rebuilt by the append double.
            let array = Rc::new(RefCell::new(Vec::<u32>::new()));
            let target = array.clone();
            w.e.register_double(0x007c_b2e0, move |e, a| {
                target.borrow_mut().push(e.mem.u32(a[1]));
                Ret::default()
            });
            w.e.register_double(0x0047_0470, move |e, a| {
                e.mem.set_u32(a[0], 0);
                e.mem.set_u32(a[0] + 4, 0);
                Ret::default()
            });
            w.e.register_double(0x005a_e3d0, move |e, a| {
                if e.mem.u32(a[0]) == 0 {
                    e.mem.set_u32(a[0], e.mem.u32(a[1]));
                }
                Ret::default()
            });
            let target = array.clone();
            w.e.register_double(0x0071_9b20, move |e, a| {
                let item = e.mem.u32(a[1]);
                ret(target
                    .borrow()
                    .iter()
                    .position(|v| *v == item)
                    .map_or(u32::MAX, |p| p as u32))
            });
            let target = array.clone();
            w.e.register_double(0x009a_4320, move |_, a| {
                target.borrow_mut().remove(a[1] as usize);
                Ret::default()
            });
            let target = array.clone();
            w.e.register_double(0x0044_ddc0, move |_, _| ret(target.borrow().len() as u32));
            started(&mut w.e);
            w.e.call(0x0060_9220, &args![w.npc, w.buffer]);
            assert_eq!(w.e.mem.u32(list), loaded);
            assert_eq!(calls(&mut w.e, 0x0060_ba40).len(), 1);
            assert_eq!(calls(&mut w.e, 0x0060_bae0).len(), 1);
            if differs {
                assert_eq!(calls(&mut w.e, 0x005f_20a0), vec![vec![w.npc.addr(), 8]]);
            } else {
                assert!(calls(&mut w.e, 0x005f_20a0).is_empty());
            }
        }
    }

    #[test]
    fn load_applies_the_sex_bit_and_the_load_flag() {
        let mut w = save_load_world(0x100_0000, 0);
        w.bytes.borrow_mut().push_back(vec![1]);
        started(&mut w.e);
        w.e.call(0x0060_9220, &args![w.npc, w.buffer]);
        assert_eq!(
            calls(&mut w.e, 0x0047_dd50),
            vec![vec![w.npc.addr() + COMPONENT_ACTOR_BASE_DATA, 1, 1, 1]]
        );
        assert_eq!(calls(&mut w.e, 0x005f_20a0), vec![vec![w.npc.addr(), 0x1b]]);
        // The stored bit equals the NPC's: nothing is set.
        let mut w = save_load_world(0x100_0000, 0);
        w.bytes.borrow_mut().push_back(vec![0]);
        started(&mut w.e);
        w.e.call(0x0060_9220, &args![w.npc, w.buffer]);
        assert!(calls(&mut w.e, 0x0047_dd50).is_empty());
        assert!(calls(&mut w.e, 0x005f_20a0).is_empty());
    }

    #[test]
    fn revert_undoes_what_the_buffer_no_longer_carries() {
        // Race: loaded flag but no saved flag -> the original race returns.
        let mut e = world();
        let npc = new_npc(&mut e);
        let race = e.mem.alloc(0x600);
        e.mem.set_u32(npc.addr() + COMPONENT_RACE + 4, race);
        let original = e.mem.alloc(0x600);
        e.set(npc, TESNPC::pOriginalRace, Ptr::new(original));
        e.mem.set_f32(race + 0x60, 1.0);
        e.set(npc, TESNPC::fHeight, 1.0);
        set_change_flags(&mut e, 0x200_0000 | 0x800 | 0x100_0000, 0);
        let alternate = e.mem.alloc(0x80);
        e.set(npc, TESNPC::pAlternateFaceOffsetCoord, Ptr::new(alternate));
        e.set(npc, TESNPC::iActorBaseFlags, 1);
        let buffer = e.mem.alloc(0x40);
        started(&mut e);
        e.call(0x0060_99f0, &args![npc, buffer]);
        assert_eq!(calls(&mut e, 0x009d_ace0), vec![vec![npc.addr(), buffer]]);
        assert_eq!(e.mem.u32(npc.addr() + COMPONENT_RACE + 4), original);
        assert!(e.get(npc, TESNPC::pOriginalRace).is_null());
        assert_eq!(calls(&mut e, 0x005d_9ff0), vec![vec![alternate, 3]]);
        assert!(e.get(npc, TESNPC::pAlternateFaceOffsetCoord).is_null());
        assert_eq!(calls(&mut e, 0x0047_0470), vec![vec![npc.addr() + 0x1dc]]);
        // The sex bit (set) is cleared: SetFlagBit(1, true, 1).
        assert_eq!(
            calls(&mut e, 0x0047_dd50),
            vec![vec![npc.addr() + COMPONENT_ACTOR_BASE_DATA, 1, 0, 1]]
        );
        assert_eq!(calls(&mut e, 0x005d_d560), vec![vec![npc.addr()]]);
        assert_eq!(calls(&mut e, 0x005f_20a0), vec![vec![npc.addr(), 0x1b]]);
        // The saved flags are also set: nothing is undone.
        set_change_flags(
            &mut e,
            0x200_0000 | 0x800 | 0x100_0000,
            0x200_0000 | 0x800 | 0x100_0000,
        );
        e.set(npc, TESNPC::pOriginalRace, Ptr::new(original));
        e.call_log = Some(vec![]);
        e.call(0x0060_99f0, &args![npc, buffer]);
        assert!(calls(&mut e, 0x005f_20a0).is_empty());
        assert!(calls(&mut e, 0x005d_9ff0).is_empty());
    }

    /// A world for `LoadFaceGen`: the old load calls hand back queued bytes
    /// and ids.
    fn old_load_world() -> SaveLoadWorld {
        let mut w = save_load_world(0, 0);
        let queue = w.bytes.clone();
        w.e.register_double(0x0048_4d00, move |e, a| {
            let next = queue.borrow_mut().pop_front().expect("a queued load");
            assert!(next.len() as u32 <= a[2]);
            e.mem.write(a[1], &next);
            Ret::default()
        });
        let queue = w.ids.clone();
        w.e.register_double(0x0048_4d40, move |e, a| {
            let id = queue.borrow_mut().pop_front().expect("a queued id");
            e.mem.set_u32(a[1], id);
            Ret::default()
        });
        w
    }

    /// Queues the loads of a face with no element changes: 8 zero elements,
    /// the race, hair and eyes ids, the hair length and colour, the sex bit.
    fn queue_old_face(w: &SaveLoadWorld, ids: [u32; 3], length: f32, color: u32, bit: u8) {
        let mut bytes = w.bytes.borrow_mut();
        for _ in 0..8 {
            bytes.push_back(vec![0, 0, 0, 0]);
        }
        bytes.push_back(length.to_bits().to_le_bytes().to_vec());
        bytes.push_back(color.to_le_bytes().to_vec());
        bytes.push_back(vec![bit]);
        w.ids.borrow_mut().extend(ids);
    }

    #[test]
    fn old_face_load_stops_when_nothing_differs_and_logs_missing_forms() {
        let mut w = old_load_world();
        let race = w.e.mem.alloc(0x600);
        w.e.mem.set_u32(w.npc.addr() + COMPONENT_RACE + 4, race);
        w.e.set(w.npc, TESNPC::pHair, Ptr::new(0x4001));
        w.e.set(w.npc, TESNPC::pEyeColor, Ptr::new(0x4002));
        w.e.set(w.npc, TESNPC::fHairLength, 0.5);
        w.e.set(w.npc, TESNPC::iHairColor, 0x0102_0304);
        queue_old_face(&w, [race, 0x4001, 0x4002], 0.5, 0x0102_0304, 0);
        let actor = w.e.mem.alloc(0x100);
        started(&mut w.e);
        w.e.call(0x0060_9f60, &args![w.npc, actor]);
        assert!(calls(&mut w.e, 0x005d_d560).is_empty());
        // The sex bit is always written back.
        assert_eq!(
            calls(&mut w.e, 0x0047_dd50),
            vec![vec![w.npc.addr() + COMPONENT_ACTOR_BASE_DATA, 1, 0, 1]]
        );
        assert!(calls(&mut w.e, LOG_MESSAGE).is_empty());
        // Ids that name nothing are logged with the NPC's name, and count
        // as changes.
        let mut w = old_load_world();
        w.e.register(LOOKUP_FORM, |_, _| ret(0));
        returns(&mut w.e, 0x0040_8da0, 0x7777);
        queue_old_face(&w, [0x3001, 0x3002, 0x3003], 0.5, 0, 0);
        let actor = object_with(&mut w.e, 0x0c44_0000, &[(0x1ac, 0), (0x1b0, 0)]);
        started(&mut w.e);
        w.e.call(0x0060_9f60, &args![w.npc, actor]);
        let logged = calls(&mut w.e, LOG_MESSAGE);
        assert_eq!(
            logged,
            vec![
                vec![0x0104_aa48, 0x7777],
                vec![0x0104_aa20, 0x7777, 0x3001],
                vec![0x0104_a9ec, 0x7777],
                vec![0x0104_a9c4, 0x7777, 0x3002],
                vec![0x0104_a99c, 0x7777, 0x3003],
            ]
        );
    }

    #[test]
    fn old_face_load_rebuilds_the_actors_head_when_something_changed() {
        let mut w = old_load_world();
        let race = w.e.mem.alloc(0x600);
        w.e.mem.set_u32(w.npc.addr() + COMPONENT_RACE + 4, race);
        // The stored face has another hair colour.
        queue_old_face(&w, [race, 0, 0], 0.0, 0x00ff_00ff, 1);
        let first_head = w.e.mem.alloc(0x20);
        let second_head = w.e.mem.alloc(0x20);
        let parent = object_with(&mut w.e, 0x0c40_0000, &[(0xe8, 0)]);
        let root = object_with(&mut w.e, 0x0c41_0000, &[]);
        let node = object_with(&mut w.e, 0x0c42_0000, &[(0xc, root)]);
        let actor = object_with(
            &mut w.e,
            0x0c43_0000,
            &[(0x1ac, first_head), (0x1b0, second_head)],
        );
        returns(&mut w.e, 0x0043_fcd0, node);
        returns(&mut w.e, 0x008b_70d0, 0x6100);
        returns(&mut w.e, 0x0049_6940, 0x6200);
        returns(&mut w.e, 0x0053_7bd0, 0x6300);
        w.e.register_double(0x0096_11e0, move |_, a| {
            ret(if a[0] == first_head || a[0] == second_head {
                parent
            } else {
                1
            })
        });
        returns(&mut w.e, 0x005d_9f90, 0x6400);
        started(&mut w.e);
        w.e.call(0x0060_9f60, &args![w.npc, actor]);
        // The head nodes are removed from the palette and their parent told.
        assert_eq!(
            calls(&mut w.e, 0x00a6_e8e0),
            vec![vec![second_head, 0x6300], vec![first_head, 0x6300]]
        );
        assert_eq!(
            calls(&mut w.e, slot(0x0c40_0000, 0xe8)),
            vec![vec![parent, second_head], vec![parent, first_head]]
        );
        assert_eq!(calls(&mut w.e, 0x005d_d560), vec![vec![w.npc.addr()]]);
        // The face-gen parameters go to both heads.
        let params = calls(&mut w.e, FACE_PARAMS_CONSTRUCT);
        assert_eq!(params.len(), 1);
        assert_eq!(
            calls(&mut w.e, 0x0061_41f0),
            vec![vec![race, w.npc.addr(), params[0][0], 0, 0]]
        );
        assert_eq!(
            calls(&mut w.e, APPLY_FACE_PARAMS),
            vec![
                vec![first_head, params[0][0]],
                vec![second_head, params[0][0]]
            ]
        );
        assert_eq!(
            calls(&mut w.e, FACE_PARAMS_DESTROY),
            vec![vec![params[0][0]]]
        );
        // The new colour and sex bit were stored.
        assert_eq!(w.e.get(w.npc, TESNPC::iHairColor), 0x00ff_00ff);
        assert_eq!(
            calls(&mut w.e, 0x0047_dd50),
            vec![vec![w.npc.addr() + COMPONENT_ACTOR_BASE_DATA, 1, 1, 1]]
        );
        // The head is loaded when the biped answers.
        assert_eq!(calls(&mut w.e, 0x0065_1b30).len(), 1);
    }

    #[test]
    fn first_person_model_refresh_calls_the_process_and_repositions_the_player_node() {
        let mut e = world();
        let this = new_npc(&mut e);
        e.map(0x011a_9000, 0x1000);
        let matrix: Vec<u8> = (0..0x24u8).collect();
        e.mem.write(0x011a_9448, &matrix);
        let process = object_with(&mut e, 0x0c50_0000, &[(0x464, 0)]);
        let actor = e.mem.alloc(0x100);
        e.mem.set_u32(actor + 0x68, process);
        let flag_seen = Rc::new(RefCell::new(vec![]));
        let seen = flag_seen.clone();
        e.register_double(slot(0x0c50_0000, 0x464), move |e, _| {
            seen.borrow_mut().push(e.mem.u8(0x011c_5cb4));
            Ret::default()
        });
        started(&mut e);
        e.call(0x0060_a890, &args![this, actor]);
        assert_eq!(*flag_seen.borrow(), vec![1]);
        assert_eq!(e.mem.u8(0x011c_5cb4), 0);
        assert!(calls(&mut e, 0x0095_0bb0).is_empty());
        // The player: the node under the first-person biped gets the matrix.
        e.set_global(PLAYER_SINGLETON, actor);
        returns(&mut e, 0x0095_0bb0, 0x6600);
        returns(&mut e, 0x0045_bc00, 0x6700);
        returns(&mut e, 0x0056_fac0, 0x6800);
        e.call_log = Some(vec![]);
        e.call(0x0060_a890, &args![this, actor]);
        assert_eq!(calls(&mut e, 0x0095_0bb0), vec![vec![actor, 1]]);
        assert_eq!(calls(&mut e, 0x0045_bc00), vec![vec![0x6600, 0]]);
        let rotate = calls(&mut e, 0x0056_fac0);
        assert_eq!(rotate.len(), 1);
        assert_eq!(rotate[0][0], actor);
        assert_ne!(rotate[0][2], 0x011a_9448);
        assert_eq!(e.mem.bytes(rotate[0][2], 0x24), matrix);
        assert_eq!(calls(&mut e, 0x0043_fa80), vec![vec![0x6700, 0x6800]]);
        // Without a biped node nothing is rotated.
        returns(&mut e, 0x0095_0bb0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0060_a890, &args![this, actor]);
        assert!(calls(&mut e, 0x0043_fa80).is_empty());
        assert!(calls(&mut e, 0x0045_bc00).is_empty());
    }

    /// The nodes `SwapEyes` works on.
    struct EyeWorld {
        e: Engine,
        npc: Ptr<TESNPC>,
        head: u32,
        left: u32,
        right: u32,
        texture: u32,
    }

    fn eye_world() -> EyeWorld {
        let mut e = world();
        let npc = new_npc(&mut e);
        let left = object_with(&mut e, 0x0c60_0000, &[(0xfc, 0), (0x1c, 0)]);
        let right = object_with(&mut e, 0x0c61_0000, &[(0xfc, 0), (0x1c, 0)]);
        let other = object_with(&mut e, 0x0c62_0000, &[(0x1c, 0)]);
        // The children report themselves as the node behind virtual 0x1c.
        for (object, table) in [
            (left, 0x0c60_0000u32),
            (right, 0x0c61_0000),
            (other, 0x0c62_0000),
        ] {
            returns(&mut e, slot(table, 0x1c), object);
        }
        let head = object_with(&mut e, 0x0c63_0000, &[(0x9c, 0)]);
        e.mem.set_u32(npc.addr() + 0x1c4, head);
        let names = [
            text(&mut e, "FaceGenEyeLeft"),
            text(&mut e, "FaceGenEyeRight"),
            text(&mut e, "Other"),
        ];
        let children = [left, right, other];
        returns(&mut e, 0x0043_b480, 3);
        e.register_double(0x0043_b4a0, move |_, a| ret(children[a[1] as usize]));
        e.register_double(0x0041_3f40, move |_, a| ret(a[0]));
        e.register_double(0x0043_b1b0, move |_, a| {
            ret(names[children.iter().position(|c| *c == a[0]).unwrap()])
        });
        let texture = e.mem.alloc(0x20);
        e.register_double(0x0045_68c0, move |e, a| {
            e.mem.set_u32(a[2], texture);
            Ret::default()
        });
        e.set_global(TES_SINGLETON, 0x4545u32);
        e.mem.set_cstr(0x0104_aac0, b"FaceGenEyeLeft");
        e.mem.set_cstr(0x0104_aab0, b"FaceGenEyeRight");
        returns(&mut e, 0x0050_d100, 0x6a00);
        EyeWorld {
            e,
            npc,
            head,
            left,
            right,
            texture,
        }
    }

    #[test]
    fn swap_eyes_does_nothing_without_a_biped_head_node() {
        let mut e = world();
        let npc = new_npc(&mut e);
        started(&mut e);
        e.call(0x0060_a950, &args![npc, 0u32]);
        assert_eq!(calls(&mut e, NI_POINTER_CONSTRUCT).len(), 2);
        assert_eq!(calls(&mut e, NI_POINTER_DESTROY).len(), 2);
        assert!(calls(&mut e, 0x0043_b480).is_empty());
    }

    /// Gives both eye shapes data objects of a usable size (10) that answer
    /// the virtual 0xfc; returns the data object.
    fn usable_shapes(w: &mut EyeWorld) -> u32 {
        let usable = object_with(&mut w.e, 0x0c64_0000, &[(0xfc, 0)]);
        returns(&mut w.e, 0x0043_b230, usable);
        returns(&mut w.e, 0x0044_1110, 10);
        usable
    }

    #[test]
    fn swap_eyes_retextures_usable_eye_shapes() {
        let mut w = eye_world();
        let eyes = w.e.mem.alloc(0x40);
        returns(&mut w.e, 0x0040_8da0, 0x7a7a);
        // Both shapes have data of a usable size (8 to 12).
        let usable = usable_shapes(&mut w);
        let shapes = Rc::new(RefCell::new(vec![]));
        let seen = shapes.clone();
        w.e.register_double(0x0050_d100, move |_, a| {
            seen.borrow_mut().push(a[0]);
            ret(0x6a00 + a[0] % 0x100)
        });
        started(&mut w.e);
        w.e.call(0x0060_a950, &args![w.npc, eyes]);
        assert_eq!(*shapes.borrow(), vec![w.left, w.right]);
        // The path is built from the eyes' texture name.
        let build = calls(&mut w.e, 0x0040_6f60);
        assert_eq!(build.len(), 1);
        assert_eq!(&build[0][1..], &[0x0104_a64c, 0x7a7a]);
        assert_eq!(calls(&mut w.e, 0x0040_8da0), vec![vec![eyes + 0x24]]);
        // The texture is created once and given to both usable nodes.
        let created = calls(&mut w.e, 0x0045_68c0);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][0], 0x4545);
        assert_eq!(
            calls(&mut w.e, slot(0x0c64_0000, 0xfc)),
            vec![vec![usable, 0, w.texture], vec![usable, 0, w.texture]]
        );
        assert!(calls(&mut w.e, 0x00aa_13e0).is_empty());
        // Nothing else: the head's eye nodes are not looked up again.
        assert!(calls(&mut w.e, slot(0x0c63_0000, 0x9c)).is_empty());
        assert_eq!(calls(&mut w.e, 0x0040_37d0).len(), 1);
        // The default texture for no eyes.
        let mut w = eye_world();
        usable_shapes(&mut w);
        started(&mut w.e);
        w.e.call(0x0060_a950, &args![w.npc, 0u32]);
        let build = calls(&mut w.e, 0x0040_6f60);
        assert_eq!(&build[0][1..], &[0x0104_aa80]);
    }

    #[test]
    fn swap_eyes_attaches_a_new_texturing_property_to_unusable_shapes() {
        let mut w = eye_world();
        // Data sizes outside 8..=12.
        returns(&mut w.e, 0x0043_b230, 0x6b00);
        returns(&mut w.e, 0x0044_1110, 3);
        let memory = w.e.mem.alloc(0x30);
        returns(&mut w.e, 0x00aa_13e0, memory);
        returns(&mut w.e, 0x00a6_aa40, memory);
        // The left node already has a property of type 5.
        let left = w.left;
        w.e.register_double(0x00a5_9d30, move |_, a| ret((a[0] == left) as u32));
        // The property's slot holder: first the empty slot, then it is filled.
        let slot_cell = w.e.mem.alloc(8);
        returns(&mut w.e, 0x0087_7a30, slot_cell);
        let object = w.e.mem.alloc(0x20);
        returns(&mut w.e, 0x00a6_9dd0, object);
        started(&mut w.e);
        w.e.call(0x0060_a950, &args![w.npc, 0u32]);
        assert_eq!(calls(&mut w.e, 0x00aa_13e0), vec![vec![0x30]]);
        assert_eq!(calls(&mut w.e, 0x005b_8fc0), vec![vec![memory, w.texture]]);
        assert_eq!(calls(&mut w.e, 0x004f_3200), vec![vec![memory, 3]]);
        // fn_0060aeb0 stored a new 16-byte object and told it 0xf00/8.
        assert_eq!(calls(&mut w.e, 0x0096_ae90).len(), 1);
        assert_eq!(
            calls(&mut w.e, 0x004f_32e0),
            vec![vec![object, 2, 0xf00, 8]]
        );
        assert_eq!(calls(&mut w.e, 0x00a5_b230), vec![vec![w.left, 5]]);
        assert_eq!(
            calls(&mut w.e, 0x0043_9410),
            vec![vec![w.left, memory], vec![w.right, memory]]
        );
        // The eye nodes of the head are prepared at the end.
        assert_eq!(calls(&mut w.e, slot(0x0c63_0000, 0x9c)).len(), 2);
        assert_eq!(calls(&mut w.e, 0x0043_8170).len(), 2);
        assert_eq!(calls(&mut w.e, 0x0043_81b0).len(), 2);
        let _ = w.head;
    }

    #[test]
    fn texturing_property_helper_fills_an_empty_slot_once() {
        let mut e = world();
        let owner = e.mem.alloc(0x80);
        let slot_cell = e.mem.alloc(8);
        returns(&mut e, 0x0087_7a30, slot_cell);
        let object = e.mem.alloc(0x10);
        returns(&mut e, 0x00a6_9dd0, object);
        started(&mut e);
        e.call(0x0060_aeb0, &args![owner, 7u32]);
        assert_eq!(calls(&mut e, 0x0087_7a30), vec![vec![owner + 0x1c, 0]]);
        let stored = calls(&mut e, 0x0096_ae90);
        assert_eq!(stored.len(), 1);
        assert_eq!(&stored[0][..2], &[owner + 0x1c, 0]);
        assert_eq!(e.mem.u32(stored[0][2]), object);
        assert_eq!(calls(&mut e, 0x004f_32e0), vec![vec![object, 7, 0xf00, 8]]);
        // A filled slot is used as it is.
        let filled = e.mem.alloc(0x10);
        e.mem.set_u32(slot_cell, filled);
        e.call_log = Some(vec![]);
        e.call(0x0060_aeb0, &args![owner, 9u32]);
        assert!(calls(&mut e, 0x0096_ae90).is_empty());
        assert_eq!(calls(&mut e, 0x004f_32e0), vec![vec![filled, 9, 0xf00, 8]]);
        // And the plain forwarder.
        e.call_log = Some(vec![]);
        e.call(0x0060_af60, &args![0x1000u32, 0x1_0005u32]);
        assert_eq!(calls(&mut e, 0x004f_32e0), vec![vec![0x1000, 5, 0xf00, 8]]);
    }

    #[test]
    fn default_model_list_collects_the_biped_models_and_the_extra_one() {
        let mut e = world();
        let npc = new_npc(&mut e);
        e.set(npc, TESNPC::iActorBaseFlags, 1);
        let items: Vec<u32> = (0..6).map(|_| e.mem.alloc(0x100)).collect();
        let held = items.clone();
        // Items in slots 0, 2 and 5; slot 5's is covered by slot 2's model.
        e.register_double(0x0048_29c0, move |_, a| {
            ret(match a[2] {
                0 => held[0],
                2 => held[2],
                5 => held[5],
                _ => 0,
            })
        });
        e.register_double(0x0048_11e0, |_, a| ret(0x9000 + a[1] + (a[0] & 0xfff)));
        let covering = items[2] + 0x70;
        e.register_double(0x0048_0af0, move |_, a| {
            ret((a[0] == covering && a[1] == 5) as u32)
        });
        e.register_double(0x0096_a2d0, |_, a| ret(a[0]));
        let appended = Rc::new(RefCell::new(vec![]));
        let seen = appended.clone();
        e.register_double(0x005a_e3d0, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        let list = e.call(0x0060_af90, &args![npc, 1u32, 0u32]).u32();
        assert_ne!(list, 0);
        // Sex 1 (female): model = 0x9000 + 1 + low bits of the model address.
        let model = |item: u32| 0x9000 + 1 + ((item + 0x70) & 0xfff);
        assert_eq!(*appended.borrow(), vec![model(items[0]), model(items[2])]);
        // Only the extra model.
        appended.borrow_mut().clear();
        let source = e.mem.alloc(0x100);
        returns(&mut e, 0x0048_2910, source);
        // The object at `source + 0x3c` is embedded in the source: it needs a vtable.
        object_with(&mut e, 0x0c71_0000, &[(0x14, 0x9998)]);
        e.mem.set_u32(source + 0x3c, 0x0c71_0000);
        e.call(0x0060_af90, &args![npc, 0u32, 1u32]);
        assert_eq!(*appended.borrow(), vec![0x9998]);
        // No source: nothing.
        appended.borrow_mut().clear();
        returns(&mut e, 0x0048_2910, 0);
        e.call(0x0060_af90, &args![npc, 0u32, 1u32]);
        assert!(appended.borrow().is_empty());
    }

    #[test]
    fn actor_flag_testers_ask_for_the_0x800_and_0x1000_bits() {
        let mut e = world();
        let flags = block(&mut e, 0x20, &[(4, 0x800)]);
        assert_eq!(e.call(0x0060_b1d0, &args![flags]).u32(), 1);
        assert_eq!(e.call(0x0060_b1f0, &args![flags]).u32(), 0);
        e.mem.set_u32(flags + 4, 0x1000);
        assert_eq!(e.call(0x0060_b1d0, &args![flags]).u32(), 0);
        assert_eq!(e.call(0x0060_b1f0, &args![flags]).u32(), 1);
    }

    const ACT_TARGET: u32 = 0x0c80_0000;
    const ACT_ACTIVATOR: u32 = 0x0c81_0000;
    const ACT_TARGET_PROCESS: u32 = 0x0c82_0000;
    const ACT_ACTIVATOR_PROCESS: u32 = 0x0c83_0000;

    /// Two actors with processes whose virtual slots all answer 0 unless a
    /// test changes them with `returns(.., slot(TABLE, offset), value)`.
    struct ActivateWorld {
        e: Engine,
        this: Ptr<TESNPC>,
        target: u32,
        activator: u32,
        target_process: u32,
        activator_process: u32,
        position: u32,
    }

    fn zero_slots(offsets: &[u32]) -> Vec<(u32, u32)> {
        offsets.iter().map(|o| (*o, 0)).collect()
    }

    fn activate_world() -> ActivateWorld {
        let mut e = world();
        let this = new_npc(&mut e);
        e.mem.set_u32(0x0101_62c0, 2.0f32.to_bits());
        let position = block(&mut e, 0x10, &[(0, 0x11), (4, 0x22), (8, 0x33)]);
        let mut target_slots = zero_slots(&[
            0x2e8, 0x22c, 0x230, 0x234, 0x304, 0x428, 0x214, 0x218, 0x17c, 0x418, 0x48, 0x2c8,
        ]);
        target_slots.push((0x1f4, position));
        let mut activator_slots = zero_slots(&[0x28c, 0x218, 0x280, 0x214, 0x304]);
        activator_slots.push((0x1f4, position));
        let process_slots = zero_slots(&[
            0x52c, 0x610, 0x27c, 0x5a0, 0x600, 0x4d4, 0x614, 0x294, 0x288, 0x33c, 0x110, 0x604,
            0x3fc, 0x4ec,
        ]);
        let target_process = object_with(&mut e, ACT_TARGET_PROCESS, &process_slots);
        let activator_process = object_with(&mut e, ACT_ACTIVATOR_PROCESS, &process_slots);
        let target = object_with(&mut e, ACT_TARGET, &target_slots);
        let activator = object_with(&mut e, ACT_ACTIVATOR, &activator_slots);
        e.mem.set_u32(target + 0x68, target_process);
        e.mem.set_u32(activator + 0x68, activator_process);
        ActivateWorld {
            e,
            this,
            target,
            activator,
            target_process,
            activator_process,
            position,
        }
    }

    impl ActivateWorld {
        fn activate(&mut self, item: u32, count: u32) -> bool {
            let (this, target, activator) = (self.this, self.target, self.activator);
            self.e
                .call(
                    0x0060_7990,
                    &args![this, target, activator, 0u32, item, count],
                )
                .bool()
        }

        /// Sets what virtual slot `offset` of one of the objects answers.
        fn answer(&mut self, table: u32, offset: u32, value: u32) {
            returns(&mut self.e, slot(table, offset), value);
        }
    }

    #[test]
    fn activate_refuses_what_is_not_a_live_actor_with_a_free_process() {
        // Dead or disabled (virtual 0x2e8).
        let mut w = activate_world();
        w.answer(ACT_TARGET, 0x2e8, 1);
        assert!(!w.activate(0, 0));
        // No process.
        let mut w = activate_world();
        w.e.mem.set_u32(w.target + 0x68, 0);
        assert!(!w.activate(0, 0));
        // Process virtual 0x610 set.
        let mut w = activate_world();
        w.answer(ACT_TARGET_PROCESS, 0x610, 1);
        assert!(!w.activate(0, 0));
        // Process virtual 0x52c set and the target not answering 0x22c.
        let mut w = activate_world();
        w.answer(ACT_TARGET_PROCESS, 0x52c, 1);
        assert!(!w.activate(0, 0));
        // ... but answering it passes this test (and goes on to the rest).
        w.answer(ACT_TARGET, 0x22c, 1);
        w.e.register(0x004f_8960, |_, _| ret(5));
        assert!(w.activate(0, 0));
    }

    #[test]
    fn activating_the_downed_player_moves_the_clock_back() {
        let mut w = activate_world();
        w.e.set_global(PLAYER_SINGLETON, w.target);
        w.e.mem.set_u8(w.target + 0x20c, 1);
        w.answer(ACT_TARGET, 0x304, 1);
        returns(&mut w.e, 0x008a_61b0, 1);
        w.e.register(0x0086_7da0, |_, _| ret_float(10.5));
        started(&mut w.e);
        assert!(!w.activate(0, 0));
        assert_eq!(
            calls(&mut w.e, 0x0069_3d50),
            vec![vec![w.target_process, 9.5f32.to_bits()]]
        );
        assert_eq!(calls(&mut w.e, 0x0086_7da0), vec![vec![0x011d_e7b8]]);
        // Without the alarm test nothing is moved.
        w.answer(ACT_TARGET, 0x304, 0);
        w.e.call_log = Some(vec![]);
        assert!(!w.activate(0, 0));
        assert!(calls(&mut w.e, 0x0069_3d50).is_empty());
    }

    #[test]
    fn activate_shows_a_message_for_a_target_that_is_asleep_dead_or_fleeing() {
        // Down (virtual 0x22c) and of kind 6: the surprised icon.
        let mut w = activate_world();
        w.answer(ACT_TARGET, 0x22c, 1);
        w.e.register(0x004f_8960, |_, _| ret(6));
        returns(&mut w.e, 0x0040_3df0, 0x1111);
        returns(&mut w.e, 0x0055_d520, 0x2222);
        started(&mut w.e);
        assert!(!w.activate(0, 0));
        assert_eq!(calls(&mut w.e, 0x0040_3df0), vec![vec![0x011d_210c]]);
        let formatted = calls(&mut w.e, 0x00ec_623a);
        assert_eq!(formatted.len(), 1);
        assert_eq!(&formatted[0][1..], &[0x0101_2058, 0x2222, 0x1111]);
        let displayed = calls(&mut w.e, 0x0070_52f0);
        assert_eq!(
            displayed,
            vec![vec![
                formatted[0][0],
                0,
                0x0104_a958,
                0,
                2.0f32.to_bits(),
                0
            ]]
        );
        // The 0x230 test alone gives the same message.
        let mut w = activate_world();
        w.answer(ACT_TARGET, 0x230, 1);
        started(&mut w.e);
        assert!(!w.activate(0, 0));
        assert_eq!(calls(&mut w.e, 0x0070_52f0)[0][2], 0x0104_a958);
        // Fleeing (008a6650) with no combat target: the sad icon.
        let mut w = activate_world();
        returns(&mut w.e, 0x008a_6650, 1);
        started(&mut w.e);
        assert!(!w.activate(0, 0));
        assert_eq!(calls(&mut w.e, 0x0040_3df0), vec![vec![0x011d_2538]]);
        assert_eq!(calls(&mut w.e, 0x0070_52f0).len(), 1);
        assert_eq!(calls(&mut w.e, 0x0070_52f0)[0][2], 0x0102_08a0);
        // Fleeing but the combat target is the player and not blocked:
        // no message.
        let mut w = activate_world();
        let player = w.e.mem.alloc(0x40);
        w.e.set_global(PLAYER_SINGLETON, player);
        returns(&mut w.e, 0x008a_6650, 1);
        w.answer(ACT_TARGET, 0x428, 0x7777);
        returns(&mut w.e, 0x0097_fa10, 1);
        returns(&mut w.e, 0x0089_4d60, 0);
        started(&mut w.e);
        assert!(w.activate(0, 0));
        assert!(calls(&mut w.e, 0x0070_52f0).is_empty());
        // Blocked: the message after all.
        returns(&mut w.e, 0x0089_4d60, 1);
        w.e.call_log = Some(vec![]);
        assert!(!w.activate(0, 0));
        assert_eq!(calls(&mut w.e, 0x0070_52f0).len(), 1);
    }

    #[test]
    fn activate_opens_the_companion_menu_for_the_player() {
        let mut w = activate_world();
        w.e.set_global(PLAYER_SINGLETON, w.activator);
        returns(&mut w.e, 0x0056_6950, 1);
        returns(&mut w.e, 0x0075_4d90, 1);
        started(&mut w.e);
        assert!(!w.activate(0, 0));
        assert_eq!(
            calls(&mut w.e, 0x0070_9470),
            vec![vec![8, w.target, 0, 0, 0, 0]]
        );
        assert_eq!(calls(&mut w.e, 0x0075_4d90), vec![vec![w.target]]);
        // The wheel declines: no menu.
        returns(&mut w.e, 0x0075_4d90, 0);
        w.e.call_log = Some(vec![]);
        assert!(!w.activate(0, 0));
        assert!(calls(&mut w.e, 0x0070_9470).is_empty());
    }

    #[test]
    fn activate_hands_a_package_target_to_an_npc_activator_and_reports_the_day() {
        let mut w = activate_world();
        let (a, t) = (w.activator, w.target);
        w.e.register_double(0x005d_43c0, move |_, args| {
            ret(if args[0] == a { 0x6001 } else { 0x6002 })
        });
        returns(&mut w.e, 0x0041_cb10, 0x6100);
        w.e.register(0x0041_ca90, |_, args| {
            ret(if args[0] == 0x6100 { 0xf } else { 0 })
        });
        returns(&mut w.e, 0x0041_cb70, 0x6200);
        returns(&mut w.e, 0x0067_0f90, 1);
        w.e.register(0x0086_7d60, |_, _| ret(21));
        started(&mut w.e);
        assert!(w.activate(0, 0));
        assert_eq!(
            calls(&mut w.e, 0x0041_c930),
            vec![vec![0x6001, 0x6100, 4, 0x6200, 1, 1, 0]]
        );
        assert_eq!(calls(&mut w.e, 0x0041_cb70), vec![vec![0x6002]]);
        assert_eq!(
            calls(&mut w.e, slot(ACT_ACTIVATOR, 0x28c)),
            vec![vec![a, 0x6100, 21]]
        );
        assert_eq!(
            calls(&mut w.e, slot(ACT_TARGET_PROCESS, 0x5a0)),
            vec![vec![w.target_process, a, 0x6100]]
        );
        assert_eq!(
            calls(&mut w.e, slot(ACT_TARGET, 0x48)),
            vec![vec![t, 0x8000_0000]]
        );
        // The activator is told 0x4ec(1) through its process.
        assert_eq!(
            calls(&mut w.e, slot(ACT_ACTIVATOR_PROCESS, 0x4ec)),
            vec![vec![w.activator_process, 1]]
        );
        // The package is not "once per day": no day report.
        returns(&mut w.e, 0x0067_0f90, 0);
        w.e.call_log = Some(vec![]);
        assert!(w.activate(0, 0));
        assert!(calls(&mut w.e, slot(ACT_ACTIVATOR, 0x28c)).is_empty());
    }

    #[test]
    fn activate_quits_vats_and_opens_the_menu_for_the_player_facing_a_vats_target() {
        let mut w = activate_world();
        w.e.set_global(PLAYER_SINGLETON, w.activator);
        // The raw activator argument differs from the actor it casts to.
        let (a, raw) = (w.activator, 0x7001u32);
        w.e.register_double(RT_DYNAMIC_CAST, move |_, args| {
            ret(if args[0] == raw { a } else { args[0] })
        });
        returns(&mut w.e, 0x0049_3bb0, 1);
        returns(&mut w.e, 0x0056_6950, 1);
        started(&mut w.e);
        let this = w.this;
        let target = w.target;
        assert!(w
            .e
            .call(0x0060_7990, &args![this, target, raw, 0u32, 0u32, 0u32])
            .bool());
        assert_eq!(calls(&mut w.e, 0x009c_8950), vec![vec![0x011f_2250, 0, 0]]);
        assert_eq!(
            calls(&mut w.e, 0x0070_9470),
            vec![vec![4, target, 0, 0, 1, 0]]
        );
    }

    #[test]
    fn activate_lets_npcs_attack_or_steal_through_their_process() {
        // An NPC activator: its process' virtual 0x33c answers.
        let mut w = activate_world();
        returns(&mut w.e, 0x008b_06d0, 1);
        w.answer(ACT_ACTIVATOR_PROCESS, 0x33c, 1);
        started(&mut w.e);
        assert!(w.activate(0, 0));
        assert_eq!(
            calls(&mut w.e, slot(ACT_ACTIVATOR_PROCESS, 0x33c)),
            vec![vec![
                w.activator_process,
                w.activator,
                w.target,
                0,
                1,
                0,
                0,
                0,
                1,
                0,
                0,
                0,
                1,
                0
            ]]
        );
        let asked = calls(&mut w.e, 0x008b_06d0);
        assert_eq!(asked.len(), 1);
        assert_eq!(
            (asked[0][0], asked[0][1], asked[0][2], asked[0][4]),
            (w.activator, w.target, 0, 0)
        );
        // The player activator: the target's process is asked (virtual 0x33c)
        // with the roles swapped.
        let mut w = activate_world();
        w.e.set_global(PLAYER_SINGLETON, w.activator);
        returns(&mut w.e, 0x008b_06d0, 1);
        w.answer(ACT_TARGET_PROCESS, 0x33c, 1);
        started(&mut w.e);
        assert!(w.activate(0, 0));
        assert_eq!(
            calls(&mut w.e, slot(ACT_TARGET_PROCESS, 0x33c)),
            vec![vec![
                w.target_process,
                w.target,
                w.activator,
                1,
                1,
                0,
                0,
                0,
                1,
                0,
                0,
                0,
                1,
                0
            ]]
        );
        // 004997b0 on the activator stops it.
        returns(&mut w.e, 0x0049_97b0, 1);
        w.e.call_log = Some(vec![]);
        w.activate(0, 0);
        assert!(calls(&mut w.e, slot(ACT_TARGET_PROCESS, 0x33c)).is_empty());
    }

    #[test]
    fn activate_refuses_talking_to_a_dead_target_and_takes_the_dialogue_branch() {
        // Player activator, target dead (0x230) and 0087f3d0: refused.
        let mut w = activate_world();
        w.e.set_global(PLAYER_SINGLETON, w.activator);
        w.answer(ACT_TARGET, 0x230, 1);
        returns(&mut w.e, 0x0087_f3d0, 1);
        // (the dead target also triggers the first-stage message path)
        w.e.register(0x004f_8960, |_, _| ret(5));
        returns(&mut w.e, 0x008a_ce90, 0);
        assert!(!w.activate(0, 0));
    }

    /// A topic and a dialogue item for the dialogue tests.
    fn dialogue_for(w: &mut ActivateWorld) -> u32 {
        let dialogue = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(dialogue + 0xc, 0x8002);
        returns(&mut w.e, 0x0061_a2d0, 0x8001);
        returns(&mut w.e, 0x0061_b320, dialogue);
        returns(&mut w.e, 0x0061_9df0, 1);
        returns(&mut w.e, 0x0083_c7b0, 1);
        returns(&mut w.e, 0x0083_c7e0, 0);
        dialogue
    }

    #[test]
    fn activate_starts_a_dialogue_topic_with_the_player() {
        let mut w = activate_world();
        w.e.set_global(PLAYER_SINGLETON, w.activator);
        let dialogue = dialogue_for(&mut w);
        started(&mut w.e);
        assert!(w.activate(0, 0));
        assert_eq!(calls(&mut w.e, 0x0061_a2d0), vec![vec![0, 0]]);
        assert_eq!(
            calls(&mut w.e, 0x0061_b320),
            vec![vec![0x8001, w.target, w.activator, 0, 0, 0]]
        );
        assert_eq!(calls(&mut w.e, 0x0061_9df0), vec![vec![0x8002]]);
        // The say call gets the topic (0x8001), not the dialogue item: with the item the next
        // conversation walked it as a topic and faulted (0060840b, the maintainer's crash when
        // talking to Doc a second time).
        assert_eq!(
            calls(&mut w.e, 0x0057_b7c0),
            vec![vec![w.target, 0x8001, 0, 0]]
        );
        assert_eq!(calls(&mut w.e, 0x005c_90d0), vec![vec![dialogue, 1]]);
        // A target in package kind 9 answers through its process instead.
        w.answer(ACT_TARGET, 0x214, 9);
        w.e.call_log = Some(vec![]);
        assert!(w.activate(0, 0));
        assert!(calls(&mut w.e, 0x0057_b7c0).is_empty());
        assert_eq!(
            calls(&mut w.e, slot(ACT_TARGET_PROCESS, 0x600)),
            vec![vec![w.target_process, 1]]
        );
        assert_eq!(
            calls(&mut w.e, slot(ACT_TARGET, 0x418)),
            vec![vec![w.target]]
        );
        assert!(calls(&mut w.e, 0x005c_90d0).is_empty());
    }

    #[test]
    fn activate_opens_the_conversation_menu_for_the_player_without_a_topic() {
        let mut w = activate_world();
        w.e.set_global(PLAYER_SINGLETON, w.activator);
        returns(&mut w.e, 0x0045_3a70, 0x9000);
        started(&mut w.e);
        assert!(w.activate(0, 0));
        // The target turns to the player (virtual 0x214 answers 0) and is told
        // 0x294(target) through its process; both stop attacking.
        assert_eq!(
            calls(&mut w.e, 0x008b_b520),
            vec![vec![w.target, 0x11, 0x22, 0x33, 0]]
        );
        assert_eq!(
            calls(&mut w.e, slot(ACT_TARGET_PROCESS, 0x294)),
            vec![vec![w.target_process, w.target]]
        );
        assert_eq!(
            calls(&mut w.e, 0x008a_8e50),
            vec![vec![w.activator], vec![w.target]]
        );
        assert_eq!(calls(&mut w.e, 0x00ad_8780), vec![vec![0x9000, 4]]);
        assert_eq!(calls(&mut w.e, 0x0081_5b00), vec![vec![w.target + 0x88]]);
        assert_eq!(calls(&mut w.e, 0x009c_8950), vec![vec![0x011f_2250, 0, 0]]);
        // The menu is made for the NPC (the player activates), type 4.
        assert_eq!(
            calls(&mut w.e, 0x0070_9470),
            vec![vec![4, w.target, 0, 0, 1, 0]]
        );
        // The dialogue package's data goes along when there is one.
        let package = w.e.mem.alloc(0x100);
        w.e.mem.set_u32(package + 0x8c, 0x4141);
        returns(&mut w.e, 0x0093_44a0, package);
        w.e.call_log = Some(vec![]);
        assert!(w.activate(0, 0));
        assert_eq!(
            calls(&mut w.e, 0x0070_9470),
            vec![vec![4, w.target, 0x4141, 0, 1, 0]]
        );
        // A target in package kind 9 is told 0x600(1) and 0x418 instead.
        w.answer(ACT_TARGET, 0x214, 9);
        w.e.call_log = Some(vec![]);
        assert!(w.activate(0, 0));
        assert!(calls(&mut w.e, 0x0070_9470).is_empty());
        assert_eq!(calls(&mut w.e, slot(ACT_TARGET, 0x418)).len(), 1);
    }

    #[test]
    fn activate_lets_an_npc_start_a_conversation_with_the_player() {
        let mut w = activate_world();
        // The target is the player; the activator is an NPC.
        w.e.set_global(PLAYER_SINGLETON, w.target);
        returns(&mut w.e, 0x0067_8610, 1);
        w.answer(ACT_TARGET_PROCESS, 0x3fc, 1);
        started(&mut w.e);
        assert!(w.activate(0, 0));
        assert_eq!(
            calls(&mut w.e, slot(ACT_TARGET_PROCESS, 0x3fc)),
            vec![vec![w.target_process, w.activator]]
        );
        assert_eq!(
            calls(&mut w.e, slot(ACT_ACTIVATOR_PROCESS, 0x288)),
            vec![vec![w.activator_process, w.activator, 1]]
        );
        assert_eq!(calls(&mut w.e, 0x008a_7a90), vec![vec![w.target]]);
        assert_eq!(
            calls(&mut w.e, slot(ACT_ACTIVATOR_PROCESS, 0x614)),
            vec![vec![w.activator_process, 0x400]]
        );
        assert_eq!(
            calls(&mut w.e, 0x0070_9470),
            vec![vec![4, w.activator, 0, 0, 1, 0]]
        );
        // The player declines (0x3fc false): nothing happens.
        w.answer(ACT_TARGET_PROCESS, 0x3fc, 0);
        w.e.call_log = Some(vec![]);
        assert!(!w.activate(0, 0));
        assert!(calls(&mut w.e, 0x0070_9470).is_empty());
        // When the menu is already open (00702640 = 0x3f1) nothing is asked.
        returns(&mut w.e, 0x0070_2640, 0x3f1);
        w.e.call_log = Some(vec![]);
        assert!(w.activate(0, 0));
        assert!(calls(&mut w.e, slot(ACT_TARGET_PROCESS, 0x3fc)).is_empty());
    }

    #[test]
    fn activate_hands_over_an_item_or_changes_the_packages_of_two_npcs() {
        // An item that is not an NPC goes through virtual 0x17c, then 008c00e0.
        let mut w = activate_world();
        w.e.register(RT_DYNAMIC_CAST, |_, args| {
            ret(if args[3] == TYPE_TES_NPC { 0 } else { args[0] })
        });
        started(&mut w.e);
        assert!(w.activate(0x5151, 3));
        assert_eq!(
            calls(&mut w.e, slot(ACT_TARGET, 0x17c)),
            vec![vec![w.target, 0x5151, 0, 3, 1, 0, w.activator, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls(&mut w.e, 0x008c_00e0),
            vec![vec![w.activator, w.target, 0x5151, 3]]
        );
        // The activator in a package of kind 2 gets 0x288(a, 2) and stops.
        let mut w = activate_world();
        returns(&mut w.e, 0x0093_44a0, 0x5200);
        w.e.register(0x0041_ca90, |_, _| ret(2));
        started(&mut w.e);
        assert!(w.activate(0, 0));
        assert_eq!(
            calls(&mut w.e, slot(ACT_ACTIVATOR_PROCESS, 0x288)),
            vec![vec![w.activator_process, w.activator, 2]]
        );
        assert!(calls(&mut w.e, 0x008b_b520).is_empty());
        // Both in other packages: each is set to 1, they face each other and
        // the activator's 0x280 answer sets both to 2.
        let mut w = activate_world();
        returns(&mut w.e, 0x0093_44a0, 0x5200);
        w.e.register(0x0041_ca90, |_, _| ret(3));
        w.answer(ACT_ACTIVATOR, 0x218, 1);
        w.answer(ACT_TARGET, 0x218, 1);
        w.answer(ACT_ACTIVATOR, 0x280, 1);
        started(&mut w.e);
        assert!(w.activate(0, 0));
        let activator_calls = calls(&mut w.e, slot(ACT_ACTIVATOR_PROCESS, 0x288));
        assert_eq!(
            activator_calls,
            vec![
                vec![w.activator_process, w.activator, 1],
                vec![w.activator_process, w.activator, 1],
                vec![w.activator_process, w.activator, 2],
            ]
        );
        assert_eq!(
            calls(&mut w.e, slot(ACT_TARGET_PROCESS, 0x288)),
            vec![
                vec![w.target_process, w.target, 1],
                vec![w.target_process, w.target, 2]
            ]
        );
        assert_eq!(
            calls(&mut w.e, slot(ACT_ACTIVATOR, 0x280)),
            vec![vec![w.activator, w.target, 0, 0, 1, 0, 0, 0, 0, 0]]
        );
        // The target turned toward the activator: its own position block.
        assert_eq!(
            calls(&mut w.e, 0x008b_b520),
            vec![vec![w.target, 0x11, 0x22, 0x33, 0]]
        );
        let _ = w.position;
    }

    #[test]
    fn activate_gives_a_downed_target_to_the_player_or_hands_the_item_over() {
        // The target is down (virtual 0x22c): the player gets its menu 1.
        let mut w = activate_world();
        w.e.set_global(PLAYER_SINGLETON, w.activator);
        w.answer(ACT_TARGET, 0x22c, 1);
        w.e.register(0x004f_8960, |_, _| ret(5));
        started(&mut w.e);
        assert!(w.activate(0, 0));
        assert_eq!(
            calls(&mut w.e, 0x0070_9470),
            vec![vec![1, w.target, 0, 0, 1, 0]]
        );
        // An NPC activator hands the item over through the target reference.
        let mut w = activate_world();
        w.answer(ACT_TARGET, 0x22c, 1);
        w.e.register(0x004f_8960, |_, _| ret(5));
        w.e.register(RT_DYNAMIC_CAST, |_, args| {
            ret(if args[3] == TYPE_TES_NPC { 0 } else { args[0] })
        });
        started(&mut w.e);
        assert!(w.activate(0x5151, 2));
        assert_eq!(
            calls(&mut w.e, slot(ACT_TARGET, 0x17c)),
            vec![vec![w.target, 0x5151, 0, 2, 0, 0, w.activator, 0, 0, 1, 0]]
        );
    }

    #[test]
    fn activate_scolds_the_player_in_front_of_a_hostile_target_with_a_combat_target() {
        let mut w = activate_world();
        w.e.set_global(PLAYER_SINGLETON, w.activator);
        returns(&mut w.e, 0x0049_3bb0, 1);
        w.answer(ACT_TARGET, 0x428, 0x7777);
        returns(&mut w.e, 0x0047_c850, 1);
        returns(&mut w.e, 0x0040_3df0, 0x1111);
        returns(&mut w.e, 0x0040_8da0, 0x3333);
        started(&mut w.e);
        assert!(!w.activate(0, 0));
        assert_eq!(calls(&mut w.e, 0x0040_3df0), vec![vec![0x011c_f594]]);
        assert_eq!(
            calls(&mut w.e, 0x0040_8da0),
            vec![vec![w.this.addr() + COMPONENT_FULL_NAME]]
        );
        let formatted = calls(&mut w.e, 0x00ec_623a);
        assert_eq!(&formatted[0][1..], &[0x0101_2058, 0x3333, 0x1111]);
        let displayed = calls(&mut w.e, 0x0070_52f0);
        assert_eq!(displayed[0][1..], [0, 0, 0, 2.0f32.to_bits(), 0]);
        // Not a matter for the player: handled quietly.
        returns(&mut w.e, 0x0047_c850, 0);
        w.e.call_log = Some(vec![]);
        assert!(w.activate(0, 0));
        assert!(calls(&mut w.e, 0x0070_52f0).is_empty());
    }
    // END second-block tests
}

/// Tests of the third block of functions (`0060b210` to `0060bed0`).
#[cfg(test)]
mod tests_block_three {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Every callee outside this block is a double that returns zero, except
    /// those that model a `std::vector<float>`, its checked iterators and the
    /// copy / fill primitives, which behave like the exe's.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            0x0047_dd50u32,
            VECTOR_ERASE,
            VECTOR_DESTROY,
            VECTOR_DEALLOCATE,
            VECTOR_LENGTH_ERROR,
            INVALID_PARAMETER,
            0x008b_78c0,
            0x004b_0680,
            MEMSET,
            0x004e_de70,
            0x0096_afc0,
            ARRAY_GROW,
            ARRAY_STORE,
            0x006b_3eb0,
            0x0084_54f0,
            FREE_BLOCK,
            0x0060_13a0,
            MEMORY_MANAGER,
            GET_THREAD_SCRAP_HEAP,
            NPC_GET_RACE,
            GET_SEX,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(OVERLOAD_TAG, |_, _| ret(0));
        e.register(RETURN_FIRST_ARGUMENT, |_, a| ret(a[0]));
        e.register(UNWRAP_ITERATOR, |e, a| ret(e.mem.u32(a[0])));
        e.register(GET_FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x005c_a4f0, |e, a| ret((e.mem.u32(a[0]) != 0) as u32));
        e.register(GET_WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SET_WORD_AT_4, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(VECTOR_SIZE, |e, a| {
            ret((e.mem.u32(a[0] + 0x10).wrapping_sub(e.mem.u32(a[0] + 0xc)) as i32 >> 2) as u32)
        });
        e.register(VECTOR_MAX_SIZE, |_, _| ret(0x3fff_ffff));
        e.register(VECTOR_ALLOCATE, |e, a| ret(e.mem.alloc(a[1] * 4)));
        e.register(VECTOR_ELEMENT_ADDRESS, |e, a| {
            ret(e.mem.u32(a[0] + 0xc) + a[1] * 4)
        });
        // end(out): {container, last}; begin(out): {container, first}.
        e.register(VECTOR_END, |e, a| {
            let proxy = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], proxy);
            let last = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[1] + 4, last);
            ret(a[1])
        });
        e.register(VECTOR_BEGIN, |e, a| {
            let proxy = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], proxy);
            let first = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[1] + 4, first);
            ret(a[1])
        });
        e.register(UNINITIALIZED_COPY, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            e.mem.write(a[2], &bytes);
            ret(a[2] + (a[1] - a[0]))
        });
        e.register(COPY_BACKWARD, |e, a| {
            let size = a[1] - a[0];
            let bytes = e.mem.bytes(a[0], size);
            e.mem.write(a[2] - size, &bytes);
            ret(a[2] - size)
        });
        e.register(UNINITIALIZED_FILL, |e, a| {
            for i in 0..a[1] {
                let word = e.mem.u32(a[2]);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            Ret::default()
        });
        e
    }

    fn started(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A matrix whose vector holds `values` in a buffer of `capacity` floats.
    fn matrix(e: &mut Engine, values: &[f32], capacity: u32) -> Ptr<Fr2Matrix> {
        let m: Ptr<Fr2Matrix> = e.new_object();
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, m.addr());
        e.mem.set_u32(m.addr(), proxy);
        if capacity != 0 {
            let buffer = e.mem.alloc(capacity * 4);
            for (i, v) in values.iter().enumerate() {
                e.mem.set_f32(buffer + 4 * i as u32, *v);
            }
            e.set(m, Fr2Matrix::first, buffer);
            e.set(m, Fr2Matrix::last, buffer + 4 * values.len() as u32);
            e.set(m, Fr2Matrix::end_of_storage, buffer + 4 * capacity);
        }
        m
    }

    fn elements(e: &Engine, m: Ptr<Fr2Matrix>) -> Vec<f32> {
        let first = e.get(m, Fr2Matrix::first);
        let last = e.get(m, Fr2Matrix::last);
        (first..last).step_by(4).map(|p| e.mem.f32(p)).collect()
    }

    fn float_cell(e: &mut Engine, value: f32) -> Ptr {
        let cell = e.mem.alloc(4);
        e.mem.set_f32(cell, value);
        Ptr::new(cell)
    }

    fn new_npc(e: &mut Engine) -> Ptr<TESNPC> {
        let npc: Ptr<TESNPC> = e.new_object();
        e.set(npc, TESNPC::cFormType, 0x2a);
        npc
    }

    #[test]
    fn flag_bit_0x1000_is_set_through_the_actor_base_setter() {
        let mut e = engine();
        started(&mut e);
        fn_0060b210(&mut e, Ptr::new(0x1234), 1);
        assert_eq!(calls(&e, 0x0047_dd50), vec![vec![0x1234, 0x1000, 1, 1]]);
    }

    /// An NPC of race 0 (male height 1.0; race 1 has 2.0, race 2 has 3.0)
    /// with the given height and original race.
    fn race_world(height: f32, original: u32) -> (Engine, Ptr<TESNPC>, [u32; 3], u32) {
        let mut e = engine();
        e.register(NPC_GET_RACE, |e, a| ret(e.mem.u32(a[0] + 0x110)));
        let npc = new_npc(&mut e);
        let mut races = [0u32; 3];
        for (i, race) in races.iter_mut().enumerate() {
            *race = e.mem.alloc(0x80);
            e.mem.set_f32(*race + 0x60, 1.0 + i as f32);
        }
        e.mem.set_u32(npc.addr() + COMPONENT_RACE + 4, races[0]);
        e.set(npc, TESNPC::fHeight, height);
        e.set(npc, TESNPC::pOriginalRace, Ptr::new(original));
        let table = 0x0c30_0000;
        e.put_vtable(table, &[0; 0x20]);
        e.mem.set_u32(table + 0x48, 0x0c31_0048);
        e.mem.set_u32(table + 0x4c, 0x0c31_004c);
        e.mem.set_u32(npc.addr(), table);
        for target in [0x0c31_0048u32, 0x0c31_004c] {
            e.register(target, |_, _| Ret::default());
        }
        let actor = e.mem.alloc(0x10);
        (e, npc, races, actor)
    }

    #[test]
    fn race_change_to_the_same_race_does_nothing() {
        let (mut e, npc, races, actor) = race_world(1.0, 0);
        started(&mut e);
        fn_0060b240(&mut e, npc, Ptr::new(races[0]), Ptr::new(actor));
        assert!(calls(&e, SET_WORD_AT_4).is_empty());
        assert!(calls(&e, 0x008b_78c0).is_empty());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn race_change_remembers_the_first_race_and_moves_a_following_height() {
        let (mut e, npc, races, actor) = race_world(1.0, 0);
        started(&mut e);
        fn_0060b240(&mut e, npc, Ptr::new(races[1]), Ptr::new(actor));
        assert_eq!(
            calls(&e, SET_WORD_AT_4),
            vec![vec![npc.addr() + COMPONENT_RACE, races[1]]]
        );
        assert_eq!(calls(&e, 0x0c31_0048), vec![vec![npc.addr(), 0x200_0000]]);
        assert!(calls(&e, 0x0c31_004c).is_empty());
        assert_eq!(e.get(npc, TESNPC::pOriginalRace).addr(), races[0]);
        // The height was the old race's (1.0), so it now follows the new one.
        assert_eq!(e.get(npc, TESNPC::fHeight), 2.0);
        assert_eq!(calls(&e, 0x008b_78c0), vec![vec![actor, 0]]);
    }

    #[test]
    fn race_change_keeps_the_original_race_and_a_custom_height() {
        let (mut e, npc, races, _) = race_world(1.75, 0);
        e.set(npc, TESNPC::pOriginalRace, Ptr::new(races[2]));
        started(&mut e);
        fn_0060b240(&mut e, npc, Ptr::new(races[1]), Ptr::NULL);
        assert_eq!(e.get(npc, TESNPC::pOriginalRace).addr(), races[2]);
        assert_eq!(e.get(npc, TESNPC::fHeight), 1.75);
        assert!(calls(&e, 0x008b_78c0).is_empty());
    }

    #[test]
    fn race_change_back_to_the_original_race_clears_it_and_the_flag() {
        let (mut e, npc, races, actor) = race_world(1.0, 0);
        e.set(npc, TESNPC::pOriginalRace, Ptr::new(races[1]));
        started(&mut e);
        fn_0060b240(&mut e, npc, Ptr::new(races[1]), Ptr::new(actor));
        assert!(e.get(npc, TESNPC::pOriginalRace).is_null());
        assert_eq!(calls(&e, 0x0c31_004c), vec![vec![npc.addr(), 0x200_0000]]);
        assert!(calls(&e, 0x0c31_0048).is_empty());
        assert_eq!(e.mem.u32(npc.addr() + COMPONENT_RACE + 4), races[1]);
    }

    #[test]
    fn matrix_resize_stores_the_counts_and_zero_fills() {
        let mut e = engine();
        let m = matrix(&mut e, &[], 0);
        fn_0060b340(&mut e, m, 2, 3);
        assert_eq!(e.get(m, Fr2Matrix::nrows), 2);
        assert_eq!(e.get(m, Fr2Matrix::ncols), 3);
        assert_eq!(elements(&e, m), vec![0.0; 6]);
    }

    #[test]
    fn matrix_row_view_points_into_the_data_with_the_column_count() {
        let mut e = engine();
        let m = matrix(&mut e, &[0.0; 6], 6);
        e.set(m, Fr2Matrix::nrows, 2);
        e.set(m, Fr2Matrix::ncols, 3);
        let out = e.mem.alloc(8);
        started(&mut e);
        let result = fn_0060b370(&mut e, m, Ptr::new(out), 1);
        assert_eq!(result.addr(), out);
        let first = e.get(m, Fr2Matrix::first);
        assert_eq!(calls(&e, 0x004b_0680), vec![vec![out, first + 12, 3]]);
    }

    #[test]
    fn matrix_clear_zeroes_rows_times_columns_floats() {
        let mut e = engine();
        let m = matrix(&mut e, &[1.0; 6], 6);
        e.set(m, Fr2Matrix::nrows, 2);
        e.set(m, Fr2Matrix::ncols, 3);
        started(&mut e);
        fn_0060b3b0(&mut e, m);
        let first = e.get(m, Fr2Matrix::first);
        assert_eq!(calls(&e, MEMSET), vec![vec![first, 0, 24]]);
    }

    #[test]
    fn resize_to_the_same_size_does_nothing() {
        let mut e = engine();
        let m = matrix(&mut e, &[1.0, 2.0], 2);
        started(&mut e);
        fn_0060b3f0(&mut e, m, 2);
        assert_eq!(elements(&e, m), vec![1.0, 2.0]);
        assert!(calls(&e, VECTOR_END).is_empty());
        assert!(calls(&e, VECTOR_ERASE).is_empty());
    }

    #[test]
    fn resize_up_appends_the_value() {
        let mut e = engine();
        let m = matrix(&mut e, &[1.0, 2.0], 2);
        fn_0060b410(&mut e, m, 4, 7.5);
        assert_eq!(elements(&e, m), vec![1.0, 2.0, 7.5, 7.5]);
        // Zero fill through `fn_0060b3f0`.
        let m = matrix(&mut e, &[1.0], 4);
        fn_0060b3f0(&mut e, m, 3);
        assert_eq!(elements(&e, m), vec![1.0, 0.0, 0.0]);
    }

    #[test]
    fn resize_down_erases_from_begin_plus_count_to_end() {
        let mut e = engine();
        let m = matrix(&mut e, &[1.0, 2.0, 3.0, 4.0], 4);
        let first = e.get(m, Fr2Matrix::first);
        let last = e.get(m, Fr2Matrix::last);
        started(&mut e);
        fn_0060b410(&mut e, m, 1, 0.0);
        let erase = calls(&e, VECTOR_ERASE);
        assert_eq!(erase.len(), 1);
        // (this, out, first.proxy, first.pointer, last.proxy, last.pointer)
        assert_eq!(erase[0][0], m.addr());
        let proxy = e.mem.u32(m.addr());
        assert_eq!(&erase[0][2..], &[proxy, first + 4, proxy, last]);
    }

    #[test]
    fn insert_of_nothing_does_nothing() {
        let mut e = engine();
        let m = matrix(&mut e, &[1.0], 2);
        let value = float_cell(&mut e, 5.0);
        let first = e.get(m, Fr2Matrix::first);
        fn_0060b4d0(&mut e, m, 0, first, 0, value);
        assert_eq!(elements(&e, m), vec![1.0]);
    }

    #[test]
    fn insert_that_would_pass_the_maximum_size_throws() {
        let mut e = engine();
        e.register(VECTOR_MAX_SIZE, |_, _| ret(2));
        let m = matrix(&mut e, &[1.0, 2.0], 4);
        let value = float_cell(&mut e, 5.0);
        let first = e.get(m, Fr2Matrix::first);
        started(&mut e);
        fn_0060b4d0(&mut e, m, 0, first, 1, value);
        assert_eq!(calls(&e, VECTOR_LENGTH_ERROR).len(), 1);
        assert_eq!(elements(&e, m), vec![1.0, 2.0]);
    }

    #[test]
    fn insert_reallocates_to_one_and_a_half_times_or_the_exact_size() {
        let mut e = engine();
        let m = matrix(&mut e, &[1.0, 2.0, 3.0], 3);
        let value = float_cell(&mut e, 9.0);
        let old_first = e.get(m, Fr2Matrix::first);
        let old_last = e.get(m, Fr2Matrix::last);
        started(&mut e);
        fn_0060b4d0(&mut e, m, 0, old_first + 4, 2, value);
        assert_eq!(elements(&e, m), vec![1.0, 9.0, 9.0, 2.0, 3.0]);
        // 3 + 3/2 = 4 is too small for 5 elements: the exact size is used.
        assert_eq!(
            e.get(m, Fr2Matrix::end_of_storage) - e.get(m, Fr2Matrix::first),
            20
        );
        assert_eq!(
            calls(&e, VECTOR_DESTROY),
            vec![vec![m.addr(), old_first, old_last]]
        );
        assert_eq!(
            calls(&e, VECTOR_DEALLOCATE),
            vec![vec![m.addr() + 8, old_first, 3]]
        );
        // A larger capacity grows by half.
        let m = matrix(&mut e, &[1.0, 2.0, 3.0, 4.0], 4);
        let first = e.get(m, Fr2Matrix::first);
        fn_0060b4d0(&mut e, m, 0, first, 1, value);
        assert_eq!(elements(&e, m), vec![9.0, 1.0, 2.0, 3.0, 4.0]);
        assert_eq!(
            e.get(m, Fr2Matrix::end_of_storage) - e.get(m, Fr2Matrix::first),
            24
        );
        // An empty vector has nothing to destroy.
        let m = matrix(&mut e, &[], 0);
        started(&mut e);
        fn_0060b4d0(&mut e, m, 0, 0, 2, value);
        assert_eq!(elements(&e, m), vec![9.0, 9.0]);
        assert!(calls(&e, VECTOR_DESTROY).is_empty());
    }

    #[test]
    fn insert_with_a_short_tail_fills_past_the_old_end() {
        let mut e = engine();
        let m = matrix(&mut e, &[1.0, 2.0, 3.0], 8);
        let value = float_cell(&mut e, 5.0);
        let first = e.get(m, Fr2Matrix::first);
        fn_0060b4d0(&mut e, m, 0, first + 8, 5, value);
        assert_eq!(
            elements(&e, m),
            vec![1.0, 2.0, 5.0, 5.0, 5.0, 5.0, 5.0, 3.0]
        );
        // At the end: the tail is empty.
        let m = matrix(&mut e, &[1.0], 4);
        let last = e.get(m, Fr2Matrix::last);
        fn_0060b4d0(&mut e, m, 0, last, 2, value);
        assert_eq!(elements(&e, m), vec![1.0, 5.0, 5.0]);
    }

    #[test]
    fn insert_with_a_long_tail_shifts_it_up() {
        let mut e = engine();
        let m = matrix(&mut e, &[1.0, 2.0, 3.0, 4.0], 8);
        let value = float_cell(&mut e, 6.0);
        let first = e.get(m, Fr2Matrix::first);
        fn_0060b4d0(&mut e, m, 0, first + 4, 2, value);
        assert_eq!(elements(&e, m), vec![1.0, 6.0, 6.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn insert_copies_the_value_before_moving_elements_that_hold_it() {
        let mut e = engine();
        let m = matrix(&mut e, &[1.0, 2.0, 3.0, 4.0], 8);
        let first = e.get(m, Fr2Matrix::first);
        // The value is the element at index 2.
        fn_0060b4d0(&mut e, m, 0, first, 2, Ptr::new(first + 8));
        assert_eq!(elements(&e, m), vec![3.0, 3.0, 1.0, 2.0, 3.0, 4.0]);
    }

    /// An iterator (`proxy`, element pointer at the start of a `count`-float
    /// container): returns (iterator, container, first element).
    fn iterator_world(e: &mut Engine, count: u32) -> (u32, u32, u32) {
        let container = matrix(e, &vec![0.0; count as usize], count).addr();
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, container);
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, proxy);
        let first = e.mem.u32(container + 0xc);
        e.mem.set_u32(iterator + 4, first);
        (iterator, container, first)
    }

    #[test]
    fn iterator_copy_plus_count_leaves_the_original() {
        let mut e = engine();
        let (iterator, _, first) = iterator_world(&mut e, 3);
        let out = e.mem.alloc(8);
        let moved = fn_0060b820(&mut e, Ptr::new(iterator), Ptr::new(out), 2);
        assert_eq!(moved.addr(), out);
        assert_eq!(e.mem.u32(out + 4), first + 8);
        assert_eq!(e.mem.u32(out), e.mem.u32(iterator));
        assert_eq!(e.mem.u32(iterator + 4), first);
    }

    #[test]
    fn capacity_is_zero_without_a_buffer_and_the_span_with_one() {
        let mut e = engine();
        let m = matrix(&mut e, &[], 0);
        assert_eq!(fn_0060b860(&mut e, m), 0);
        let m = matrix(&mut e, &[1.0], 5);
        assert_eq!(fn_0060b860(&mut e, m), 5);
    }

    #[test]
    fn fill_n_fills_through_the_allocator_and_returns_the_end() {
        let mut e = engine();
        let m = matrix(&mut e, &[], 0);
        let buffer = e.mem.alloc(12);
        let value = float_cell(&mut e, 2.5);
        started(&mut e);
        let end = fn_0060b8a0(&mut e, m, buffer, 3, value);
        assert_eq!(end, buffer + 12);
        assert_eq!(e.mem.f32(buffer + 8), 2.5);
        assert_eq!(
            calls(&e, UNINITIALIZED_FILL),
            vec![vec![buffer, 3, value.addr()]]
        );
    }

    #[test]
    fn iterator_plus_equals_advances_and_returns_itself() {
        let mut e = engine();
        let (iterator, _, first) = iterator_world(&mut e, 3);
        let result = fn_0060b8e0(&mut e, Ptr::new(iterator), 1);
        assert_eq!(result.addr(), iterator);
        assert_eq!(e.mem.u32(iterator + 4), first + 4);
    }

    #[test]
    fn checked_advance_accepts_the_range_and_rejects_outside_it() {
        let mut e = engine();
        let (iterator, _, first) = iterator_world(&mut e, 3);
        started(&mut e);
        fn_0060b900(&mut e, Ptr::new(iterator), 3);
        assert_eq!(e.mem.u32(iterator + 4), first + 12);
        assert!(calls(&e, INVALID_PARAMETER).is_empty());
        // One past the end.
        fn_0060b900(&mut e, Ptr::new(iterator), 1);
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 1);
        // Before the beginning.
        let (iterator, _, _) = iterator_world(&mut e, 3);
        started(&mut e);
        fn_0060b900(&mut e, Ptr::new(iterator), u32::MAX);
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn checked_advance_rejects_an_iterator_without_a_container() {
        let mut e = engine();
        let (iterator, _, _) = iterator_world(&mut e, 3);
        e.register(0x005c_a4f0, |_, _| ret(0));
        started(&mut e);
        fn_0060b900(&mut e, Ptr::new(iterator), 0);
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn container_of_an_iterator_is_null_without_a_proxy() {
        let mut e = engine();
        let (iterator, container, _) = iterator_world(&mut e, 1);
        assert_eq!(fn_0060b980(&mut e, Ptr::new(iterator)), container);
        e.mem.set_u32(iterator, 0);
        assert_eq!(fn_0060b980(&mut e, Ptr::new(iterator)), 0);
    }

    #[test]
    fn array_base_destructor_restores_the_vtable_and_frees_the_buffer() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        e.set(array, NiTArray::m_pBase, 0x7000);
        started(&mut e);
        fn_0060b9b0(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_NI_T_ARRAY);
        assert_eq!(calls(&e, 0x004e_de70), vec![vec![0x7000]]);
    }

    #[test]
    fn primitive_array_constructor_adds_its_vtable_to_the_base_constructor() {
        let mut e = engine();
        e.register(0x0096_afc0, |e, _| ret(e.mem.alloc(16)));
        let array: Ptr<NiTArray> = e.new_object();
        let result = fn_0060b9e0(&mut e, array, 4, 2);
        assert_eq!(result, array);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_NI_T_PRIMITIVE_ARRAY);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 4);
        assert_eq!(e.get(array, NiTArray::m_usGrowBy), 2);
        assert_ne!(e.get(array, NiTArray::m_pBase), 0);
    }

    #[test]
    fn array_add_stores_at_the_current_size() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        e.set(array, NiTArray::m_usMaxSize, 8);
        e.set(array, NiTArray::m_usSize, 3);
        started(&mut e);
        let index = fn_0060ba10(&mut e, array, 0x55);
        assert_eq!(index, 3);
        assert_eq!(calls(&e, ARRAY_STORE), vec![vec![array.addr(), 3, 0x55]]);
        assert!(calls(&e, ARRAY_GROW).is_empty());
    }

    #[test]
    fn scrap_array_constructor_takes_the_thread_scrap_heap() {
        let mut e = engine();
        e.register(MEMORY_MANAGER, |_, _| ret(0x4444));
        e.register(GET_THREAD_SCRAP_HEAP, |_, a| ret(a[0] + 1));
        let array = e.mem.alloc(0x14);
        started(&mut e);
        let result = fn_0060ba40(&mut e, Ptr::new(array));
        assert_eq!(result.addr(), array);
        assert_eq!(e.mem.u32(array), VTABLE_BS_SCRAP_ARRAY);
        assert_eq!(e.mem.u32(array + 0x10), 0x4445);
        assert_eq!(
            calls(&e, 0x006b_3eb0),
            vec![vec![array, 0, 0], vec![array, 0, 0]]
        );
    }

    #[test]
    fn simple_array_destructor_and_constructor() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        started(&mut e);
        assert_eq!(fn_0060bdd0(&mut e, Ptr::new(array)).addr(), array);
        assert_eq!(e.mem.u32(array), VTABLE_BS_SIMPLE_ARRAY);
        assert_eq!(calls(&e, 0x006b_3eb0), vec![vec![array, 0, 0]]);
        e.mem.set_u32(array, 0);
        fn_0060bac0(&mut e, Ptr::new(array));
        assert_eq!(e.mem.u32(array), VTABLE_BS_SIMPLE_ARRAY);
        assert_eq!(calls(&e, 0x0084_54f0), vec![vec![array, 1]]);
    }

    #[test]
    fn scrap_array_destructor_runs_the_base_destructor_last() {
        let mut e = engine();
        let array = e.mem.alloc(0x14);
        started(&mut e);
        fn_0060bae0(&mut e, Ptr::new(array));
        assert_eq!(e.mem.u32(array), VTABLE_BS_SIMPLE_ARRAY);
        assert_eq!(calls(&e, 0x0084_54f0), vec![vec![array, 1], vec![array, 1]]);
    }

    #[test]
    fn uninitialized_copy_wrapper_unwraps_and_forwards_the_allocator() {
        let mut e = engine();
        let m = matrix(&mut e, &[1.0, 2.0], 2);
        let first = e.get(m, Fr2Matrix::first);
        let last = e.get(m, Fr2Matrix::last);
        let destination = e.mem.alloc(8);
        started(&mut e);
        let end = fn_0060bb40(&mut e, m, first, last, destination);
        assert_eq!(end, destination + 8);
        assert_eq!(e.mem.f32(destination + 4), 2.0);
        assert_eq!(
            calls(&e, UNINITIALIZED_COPY),
            vec![vec![first, last, destination, m.addr() + 8, 0, 0]]
        );
        assert_eq!(calls(&e, UNWRAP_ITERATOR).len(), 2);
    }

    #[test]
    fn fill_wrapper_unwraps_both_ends() {
        let mut e = engine();
        let buffer = e.mem.alloc(8);
        let value = float_cell(&mut e, 4.0);
        started(&mut e);
        fn_0060bb70(&mut e, buffer, buffer + 8, value.addr());
        assert_eq!(e.mem.f32(buffer), 4.0);
        assert_eq!(e.mem.f32(buffer + 4), 4.0);
        assert_eq!(calls(&e, UNWRAP_ITERATOR).len(), 2);
    }

    #[test]
    fn copy_backward_wrapper_unwraps_and_tags() {
        let mut e = engine();
        let buffer = e.mem.alloc(16);
        e.mem.set_f32(buffer, 1.0);
        e.mem.set_f32(buffer + 4, 2.0);
        started(&mut e);
        fn_0060bba0(&mut e, buffer, buffer + 8, buffer + 16);
        assert_eq!(e.mem.f32(buffer + 8), 1.0);
        assert_eq!(e.mem.f32(buffer + 12), 2.0);
        assert_eq!(
            calls(&e, COPY_BACKWARD),
            vec![vec![buffer, buffer + 8, buffer + 16, 0, 0, 0]]
        );
        assert_eq!(calls(&e, RETURN_FIRST_ARGUMENT).len(), 1);
    }

    #[test]
    fn fill_n_wrapper_takes_the_tag_of_its_first_argument() {
        let mut e = engine();
        let buffer = e.mem.alloc(8);
        let value = float_cell(&mut e, 1.5);
        started(&mut e);
        fn_0060bc10(&mut e, buffer, 2, value.addr(), 0x300);
        assert_eq!(e.mem.f32(buffer + 4), 1.5);
        let tags = calls(&e, OVERLOAD_TAG);
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0][0], tags[0][1]);
        assert_eq!(
            calls(&e, UNINITIALIZED_FILL),
            vec![vec![buffer, 2, value.addr()]]
        );
    }

    #[test]
    fn scalar_deleting_destructors_free_only_with_bit_zero() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        started(&mut e);
        assert_eq!(fn_0060bc60(&mut e, array, 0), array);
        assert!(calls(&e, FREE_BLOCK).is_empty());
        fn_0060bc60(&mut e, array, 1);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![array.addr()]]);
        let (second, third, fourth) = (e.mem.alloc(0x14), e.mem.alloc(0x14), e.mem.alloc(0x14));
        started(&mut e);
        fn_0060bc90(&mut e, Ptr::new(second), 3);
        assert_eq!(calls(&e, 0x0060_13a0), vec![vec![second]]);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![second]]);
        started(&mut e);
        bs_simple_array_bgs_head_part_scalar_deleting_destructor(&mut e, Ptr::new(third), 0);
        assert_eq!(calls(&e, 0x0084_54f0), vec![vec![third, 1]]);
        assert!(calls(&e, FREE_BLOCK).is_empty());
        bs_scrap_array_bgs_head_part_scalar_deleting_destructor(&mut e, Ptr::new(fourth), 1);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![fourth]]);
    }

    #[test]
    fn array_constructor_allocates_only_for_a_nonzero_maximum_size() {
        let mut e = engine();
        e.register(0x0096_afc0, |_, a| ret(0x9000 + a[0]));
        let array: Ptr<NiTArray> = e.new_object();
        e.set(array, NiTArray::m_usSize, 7);
        fn_0060bd20(&mut e, array, 5, 2);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_NI_T_ARRAY);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0x9005);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
        fn_0060bd20(&mut e, array, 0, 2);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
    }

    #[test]
    fn array_store_grows_first_when_the_index_is_past_the_maximum() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        e.set(array, NiTArray::m_usMaxSize, 4);
        e.set(array, NiTArray::m_usGrowBy, 3);
        started(&mut e);
        assert_eq!(fn_0060bd90(&mut e, array, 3, 0xaa), 3);
        assert!(calls(&e, ARRAY_GROW).is_empty());
        assert_eq!(fn_0060bd90(&mut e, array, 4, 0xbb), 4);
        assert_eq!(calls(&e, ARRAY_GROW), vec![vec![array.addr(), 7]]);
        assert_eq!(calls(&e, ARRAY_STORE).len(), 2);
    }

    #[test]
    fn uninitialized_copy_dispatch_forwards_six_words() {
        let mut e = engine();
        e.register(OVERLOAD_TAG, |_, _| ret(0x1ab));
        started(&mut e);
        let end = fn_0060be00(&mut e, 0x10, 0x10, 0x20, 0x30);
        assert_eq!(end, 0x20);
        assert_eq!(
            calls(&e, UNINITIALIZED_COPY),
            vec![vec![0x10, 0x10, 0x20, 0x30, 0xab, 0]]
        );
    }

    #[test]
    fn float_fill_copies_through_the_x87_and_quiets_signalling_nans() {
        let mut e = engine();
        let buffer = e.mem.alloc(12);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x7f80_0001);
        fn_0060be50(&mut e, buffer, buffer + 12, value);
        assert_eq!(e.mem.u32(buffer), 0x7fc0_0001);
        assert_eq!(e.mem.u32(buffer + 8), 0x7fc0_0001);
        // Ordinary values are kept bit for bit; an empty range writes nothing.
        e.mem.set_f32(value, -3.5);
        fn_0060be50(&mut e, buffer, buffer + 4, value);
        assert_eq!(e.mem.f32(buffer), -3.5);
        assert_eq!(e.mem.u32(buffer + 4), 0x7fc0_0001);
        fn_0060be50(&mut e, buffer, buffer, value);
    }

    #[test]
    fn copy_backward_dispatch_reads_the_tag_byte_and_forwards_six_words() {
        let mut e = engine();
        e.register(OVERLOAD_TAG, |_, _| ret(0x2cd));
        started(&mut e);
        e.register(UNINITIALIZED_FILL, |_, _| Ret::default());
        e.register(COPY_BACKWARD, |_, _| Ret::default());
        fn_0060be80(&mut e, 0x10, 0x18, 0x40, 0x1ee, 7, 8);
        assert_eq!(
            calls(&e, COPY_BACKWARD),
            vec![vec![0x10, 0x18, 0x40, 0xee, 0xcd, 0]]
        );
    }

    #[test]
    fn fill_n_dispatch_forwards_three_words() {
        let mut e = engine();
        e.register(UNINITIALIZED_FILL, |_, _| Ret::default());
        started(&mut e);
        fn_0060bed0(&mut e, 0x10, 2, 0x30, 1, 2, 3);
        assert_eq!(calls(&e, UNINITIALIZED_FILL), vec![vec![0x10, 2, 0x30]]);
    }

    #[test]
    fn the_translations_are_registered_under_their_addresses() {
        let mut e = Engine::new();
        assert!(e.is_translated(0x0060_b340));
        assert!(e.is_translated(0x0060_bed0));
        let m = matrix(&mut e, &[], 0);
        assert_eq!(e.call(0x0060_b860, &args![m]).u32(), 0);
    }
}
