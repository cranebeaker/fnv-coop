//! The script functions the New Vegas Script Extender (xNVSE) adds, by
//! opcode. Mods built for it compile calls to these into their scripts
//! (`SCDA`), from opcode `0x1400` on; nv-rs doesn't run them (see
//! `docs/NVSE_COVERAGE.md`).
//!
//! Data, not code: the names in the order xNVSE registers them
//! (`CommandTable::AddCommandsV1` to `V6` in `nvse/nvse/CommandTable.cpp`,
//! xNVSE commit `0ccd23ad885ddae533c1790a3fc56cd073e38de3`, 2026-09-27;
//! console commands it re-exports are named `con_<command>`), each one
//! opcode after the last. Checked against the twenty opcodes xNVSE's own
//! compiler hard-codes (`Compiler/Passes/Compiler.cpp`: `Let` 0x1539 …
//! `PluginVersion` 0x1676); see the tests.

/// The first opcode xNVSE uses (`kNVSEOpcodeStart`).
pub const FIRST_OPCODE: u16 = 0x1400;

/// Opcodes from here up belong to NVSE plugins (`kNVSEOpcodeTest`), each
/// plugin at the base it asks for (JIP LN, JohnnyGuitar and others).
pub const PLUGIN_OPCODES: u16 = 0x2000;

/// xNVSE's functions, from [`FIRST_OPCODE`] on.
pub static FUNCTIONS: [&str; 654] = [
    "GetNVSEVersion",
    "GetNVSERevision",
    "GetNVSEBeta",
    "GetBaseObject",
    "GetWeight",
    "GetHealth",
    "GetValue",
    "SetWeight",
    "SetHealth",
    "SetBaseItemValue",
    "GetType",
    "GetRepairList",
    "GetEquipType",
    "GetWeaponAmmo",
    "GetWeaponClipRounds",
    "GetAttackDamage",
    "GetWeaponType",
    "GetWeaponMinSpread",
    "GetWeaponSpread",
    "GetWeaponProjectile",
    "GetWeaponSightFOV",
    "GetWeaponMinRange",
    "GetWeaponMaxRange",
    "GetWeaponAmmoUse",
    "GetWeaponActionPoints",
    "GetWeaponCritDamage",
    "GetWeaponCritChance",
    "GetWeaponCritEffect",
    "GetWeaponFireRate",
    "GetWeaponAnimAttackMult",
    "GetWeaponRumbleLeftMotor",
    "GetWeaponRumbleRightMotor",
    "GetWeaponRumbleDuration",
    "GetWeaponRumbleWavelength",
    "GetWeaponAnimShotsPerSec",
    "GetWeaponAnimReloadTime",
    "GetWeaponAnimJamTime",
    "GetWeaponSkill",
    "GetWeaponResistType",
    "GetWeaponFireDelayMin",
    "GetWeaponFireDelayMax",
    "GetWeaponAnimMult",
    "GetWeaponReach",
    "GetWeaponIsAutomatic",
    "GetWeaponHandGrip",
    "GetWeaponReloadAnim",
    "GetWeaponBaseVATSChance",
    "GetWeaponAttackAnimation",
    "GetWeaponNumProjectiles",
    "GetWeaponAimArc",
    "GetWeaponLimbDamageMult",
    "GetWeaponSightUsage",
    "GetWeaponHasScope",
    "con_SetGameSetting",
    "con_SetINISetting",
    "con_GetINISetting",
    "con_RefreshINI",
    "con_Save",
    "con_SaveINI",
    "con_QuitGame",
    "con_LoadGame",
    "con_CloseAllMenus",
    "con_SetVel",
    "ListGetCount",
    "ListGetNthForm",
    "ListGetFormIndex",
    "ListAddForm",
    "ListAddReference",
    "ListRemoveNthForm",
    "ListRemoveForm",
    "ListReplaceNthForm",
    "ListReplaceForm",
    "ListClear",
    "GetEquippedObject",
    "GetEquippedCurrentHealth",
    "CompareNames",
    "SetName",
    "GetHotkeyItem",
    "GetNumItems",
    "GetInventoryObject",
    "SetEquippedCurrentHealth",
    "GetCurrentHealth",
    "SetCurrentHealth",
    "IsKeyPressed",
    "TapKey",
    "HoldKey",
    "ReleaseKey",
    "DisableKey",
    "EnableKey",
    "GetNumKeysPressed",
    "GetKeyPress",
    "GetNumMouseButtonsPressed",
    "GetMouseButtonPress",
    "GetControl",
    "GetAltControl",
    "MenuTapKey",
    "MenuHoldKey",
    "MenuReleaseKey",
    "DisableControl",
    "EnableControl",
    "TapControl",
    "SetControl",
    "SetAltControl",
    "SetIsControl",
    "IsControl",
    "IsKeyDisabled",
    "IsControlDisabled",
    "IsControlPressed",
    "IsPersistent",
    "GetParentCell",
    "GetParentWorldspace",
    "GetTeleportCell",
    "GetLinkedDoor",
    "GetFirstRef",
    "GetNextRef",
    "GetNumRefs",
    "GetFirstRefInCell",
    "GetNumRefsInCell",
    "GetRefCount",
    "SetRefCount",
    "GetArmorAR",
    "IsPowerArmor",
    "SetIsPowerArmor",
    "SetRepairList",
    "IsQuestItem",
    "SetQuestItem",
    "GetObjectEffect",
    "SetWeaponAmmo",
    "SetWeaponClipRounds",
    "SetAttackDamage",
    "SetWeaponType",
    "SetWeaponMinSpread",
    "SetWeaponSpread",
    "SetWeaponProjectile",
    "SetWeaponSightFOV",
    "SetWeaponMinRange",
    "SetWeaponMaxRange",
    "SetWeaponAmmoUse",
    "SetWeaponActionPoints",
    "SetWeaponCritDamage",
    "SetWeaponCritChance",
    "SetWeaponCritEffect",
    "SetWeaponAnimAttackMult",
    "SetWeaponAnimMult",
    "SetWeaponReach",
    "SetWeaponIsAutomatic",
    "SetWeaponHandGrip",
    "SetWeaponReloadAnim",
    "SetWeaponBaseVATSChance",
    "SetWeaponAttackAnimation",
    "SetWeaponNumProjectiles",
    "SetWeaponAimArc",
    "SetWeaponLimbDamageMult",
    "SetWeaponSightUsage",
    "GetNumericGameSetting",
    "SetNumericGameSetting",
    "GetNumericIniSetting",
    "SetNumericIniSetting",
    "Label",
    "Goto",
    "PrintToConsole",
    "DebugPrint",
    "SetDebugMode",
    "GetDebugMode",
    "GetUIFloat",
    "SetUIFloat",
    "SetUIString",
    "GetCrosshairRef",
    "GetGameRestarted",
    "con_ToggleMenus",
    "con_TFC",
    "con_TCL",
    "GetGameLoaded",
    "GetWeaponItemMod",
    "IsModLoaded",
    "GetModIndex",
    "GetNumLoadedMods",
    "GetSourceModIndex",
    "GetDebugSelection",
    "GetArmorDT",
    "SetArmorAR",
    "SetArmorDT",
    "IsScripted",
    "GetScript",
    "RemoveScript",
    "SetScript",
    "IsFormValid",
    "IsReference",
    "GetWeaponRequiredStrength",
    "GetWeaponRequiredSkill",
    "SetWeaponRequiredStrength",
    "SetWeaponRequiredSkill",
    "SetWeaponResistType",
    "SetWeaponSkill",
    "GetAmmoSpeed",
    "GetAmmoConsumedPercent",
    "GetAmmoCasing",
    "GetPlayerCurrentAmmoRounds",
    "SetPlayerCurrentAmmoRounds",
    "GetPlayerCurrentAmmo",
    "GetOpenKey",
    "Exp",
    "Log10",
    "Floor",
    "Ceil",
    "LeftShift",
    "RightShift",
    "LogicalAnd",
    "LogicalOr",
    "LogicalXor",
    "LogicalNot",
    "Pow",
    "Fmod",
    "Rand",
    "SortUIListBox",
    "GetOwner",
    "GetLocalRefIndex",
    "BuildRef",
    "SetNameEx",
    "MessageEx",
    "MessageBoxEx",
    "TempCloneForm",
    "IsClonedForm",
    "GetParentCellOwner",
    "GetOwningFactionRequiredRank",
    "GetParentCellOwningFactionRequiredRank",
    "SetUIStringEx",
    "con_SetUFOCamSpeedMult",
    "con_TDT",
    "SetWeaponFireRate",
    "GetWeaponLongBursts",
    "SetWeaponLongBursts",
    "GetWeaponFlags1",
    "GetWeaponFlags2",
    "SetWeaponFlags1",
    "SetWeaponFlags2",
    "GetActorBaseFlagsLow",
    "SetActorBaseFlagsLow",
    "GetActorBaseFlagsHigh",
    "SetActorBaseFlagsHigh",
    "ClearBit",
    "SetBit",
    "GetEquippedWeaponModFlags",
    "SetEquippedWeaponModFlags",
    "GetWeaponItemModEffect",
    "GetWeaponItemModValue1",
    "GetWeaponItemModValue2",
    "HasOwnership",
    "IsOwned",
    "SetOwningFactionRequiredRank",
    "GetDialogueTarget",
    "GetDialogueSubject",
    "GetDialogueSpeaker",
    "SetPackageLocationReference",
    "GetAgeClass",
    "RemoveMeIR",
    "CopyIR",
    "CreateTempRef",
    "GetFirstRefForItem",
    "GetNextRefForItem",
    "AddItemOwnership",
    "AddItemHealthPercentOwner",
    "GetTokenValue",
    "SetTokenValue",
    "GetTokenRef",
    "SetTokenRef",
    "SetTokenValueAndRef",
    "GetPaired",
    "GetRespawn",
    "SetRespawn",
    "GetPermanent",
    "SetPermanent",
    "GetBaseForm",
    "IsRefInList",
    "SetOpenKey",
    "GetCurrentPackage",
    "GetPackageLocation",
    "GetPackageCount",
    "GetNthPackage",
    "SetNthPackage",
    "AddPackageAt",
    "RemovePackageAt",
    "RemoveAllPackages",
    "ClearOpenKey",
    "SetPackageTargetReference",
    "SetPackageTargetCount",
    "GetPackageTargetCount",
    "SetPackageLocationRadius",
    "GetPackageLocationRadius",
    "SetEyes",
    "GetEyes",
    "SetHair",
    "GetHair",
    "GetHairLength",
    "SetHairLength",
    "GetHairColor",
    "SetHairColor",
    "GetNPCWeight",
    "SetNPCWeight",
    "GetNPCHeight",
    "SetNPCHeight",
    "Update3D",
    "GetVariable",
    "HasVariable",
    "GetRefVariable",
    "GetArrayVariable",
    "CompareScripts",
    "ResetAllVariables",
    "GetNumExplicitRefs",
    "GetNthExplicitRef",
    "RunScript",
    "GetCurrentScript",
    "GetCallingScript",
    "Let",
    "eval",
    "While",
    "Loop",
    "ForEach",
    "Continue",
    "Break",
    "ToString",
    "Print",
    "testexpr",
    "TypeOf",
    "Function",
    "Call",
    "SetFunctionValue",
    "GetUserTime",
    "GetModLocalData",
    "SetModLocalData",
    "ModLocalDataExists",
    "RemoveModLocalData",
    "GetAllModLocalData",
    "Internal_PushExecutionContext",
    "Internal_PopExecutionContext",
    "ar_Construct",
    "ar_Size",
    "ar_Dump",
    "ar_DumpID",
    "ar_Erase",
    "ar_Sort",
    "ar_CustomSort",
    "ar_SortAlpha",
    "ar_Find",
    "ar_First",
    "ar_Last",
    "ar_Next",
    "ar_Prev",
    "ar_Keys",
    "ar_HasKey",
    "ar_BadStringIndex",
    "ar_BadNumericIndex",
    "ar_Copy",
    "ar_DeepCopy",
    "ar_Null",
    "ar_Resize",
    "ar_Insert",
    "ar_InsertRange",
    "ar_Append",
    "ar_List",
    "ar_Map",
    "ar_Range",
    "sv_Destruct",
    "sv_Construct",
    "sv_Set",
    "sv_Compare",
    "sv_Length",
    "sv_Erase",
    "sv_SubString",
    "sv_ToNumeric",
    "sv_Insert",
    "sv_Count",
    "sv_Find",
    "sv_Replace",
    "sv_GetChar",
    "sv_Split",
    "sv_Percentify",
    "sv_ToUpper",
    "sv_ToLower",
    "IsLetter",
    "IsDigit",
    "IsPrintable",
    "IsPunctuation",
    "IsUpperCase",
    "CharToAscii",
    "ToUpper",
    "ToLower",
    "AsciiToChar",
    "NumToHex_OLD",
    "ToNumber",
    "GetNthModName",
    "GetName",
    "GetKeyName",
    "GetFormIDString",
    "GetRawFormIDString",
    "GetFalloutDirectory",
    "ActorValueToString",
    "ActorValueToStringC",
    "GetModelPath",
    "GetIconPath",
    "GetBipedModelPath",
    "GetBipedIconPath",
    "GetTexturePath",
    "SetModelPathEX",
    "SetIconPathEX",
    "SetBipedIconPathEX",
    "SetBipedModelPathEX",
    "SetTexturePath",
    "GetNthFactionRankName",
    "SetNthFactionRankNameEX",
    "GetStringGameSetting",
    "SetStringGameSettingEX_DEPRECATED",
    "GetRace",
    "GetRaceName",
    "con_SCOF",
    "PickOneOf",
    "IsPlayerSwimming",
    "GetTFC",
    "V3Length",
    "V3Normalize",
    "V3Crossproduct",
    "QFromEuler",
    "QFromAxisAngle",
    "QNormalize",
    "QMultQuatQuat",
    "QMultQuatVector3",
    "QToEuler",
    "QInterpolate",
    "IsPlayable",
    "SetIsPlayable",
    "GetEquipmentSlotsMask",
    "SetEquipmentSlotsMask",
    "con_SQV",
    "GetConsoleEcho",
    "SetConsoleEcho",
    "GetScopeModelPath",
    "SetScopeModelPath",
    "EndVATScam",
    "EquipItem2",
    "EquipMe",
    "UnequipMe",
    "IsEquipped",
    "GetInvRefsForItem",
    "SetHotkeyItem",
    "ClearHotkey",
    "PrintDebug",
    "SetVariable",
    "SetRefVariable",
    "con_ShowVars",
    "GetStringIniSetting",
    "SetStringIniSetting_DEPRECATED",
    "GetPerkRank",
    "GetAltPerkRank",
    "GetEquipmentBipedMask",
    "SetEquipmentBipedMask",
    "GetRefs",
    "GetRefsInCell",
    "GetBaseNumFactions",
    "GetBaseNthFaction",
    "GetBaseNthRank",
    "GetNumRanks",
    "GetRaceHairs",
    "GetRaceEyes",
    "GetBaseSpellListSpells",
    "GetBaseSpellListLevSpells",
    "GetBasePackages",
    "GetBaseFactions",
    "GetBaseRanks",
    "GetActiveFactions",
    "GetActiveRanks",
    "GetFactionRankNames",
    "GetFactionRankFemaleNames",
    "GetHeadParts",
    "GetLevCreatureRefs",
    "GetLevCharacterRefs",
    "GetListForms",
    "GenericAddForm",
    "GenericReplaceForm",
    "GenericDeleteForm",
    "IsPluginInstalled",
    "GetPluginVersion",
    "GenericGetForm",
    "con_INV",
    "GetNthDefaultForm",
    "SetNthDefaultForm",
    "GetDefaultForms",
    "GetGridsToLoad",
    "OutputLocalMapPicturesOverride",
    "SetOutputLocalMapPicturesGrids",
    "SetEventHandler",
    "RemoveEventHandler",
    "GetCurrentEventName",
    "DispatchEvent",
    "GetInGrid",
    "GetInGridInCell",
    "AddSpellNS",
    "GetFlagsLow",
    "SetFlagsLow",
    "GetFlagsHigh",
    "SetFlagsHigh",
    "HasConsoleOutputFilename",
    "GetConsoleOutputFilename",
    "PrintF",
    "PrintDebugF",
    "con_TFIK",
    "IsLoadDoor",
    "GetDoorTeleportX",
    "GetDoorTeleportY",
    "GetDoorTeleportZ",
    "GetDoorTeleportRot",
    "SetDoorTeleport",
    "GenericCheckForm",
    "GetEyesFlags",
    "SetEyesFlags",
    "GetHairFlags",
    "SetHairFlags",
    "ar_Packed",
    "GetActorFIKstatus",
    "SetActorFIKstatus",
    "GetBit",
    "HasEffectShader",
    "GetCurrentQuestObjectiveTeleportLinks",
    "ATan2",
    "Sinh",
    "Cosh",
    "Tanh",
    "dSin",
    "dCos",
    "dTan",
    "dASin",
    "dACos",
    "dATan",
    "dATan2",
    "dSinh",
    "dCosh",
    "dTanh",
    "GetNthAnimation",
    "AddAnimation",
    "DelAnimation",
    "DelAnimations",
    "GetClass",
    "GetNameOfClass",
    "ShowLevelUpMenu",
    "GetUIFloatAlt",
    "SetUIFloatAlt",
    "SetUIStringAlt",
    "CallAfterSeconds_OLD",
    "CallWhile_OLD",
    "CallForSeconds_OLD",
    "ar_DumpF",
    "PrintVar",
    "ar_FindWhere",
    "ar_Filter",
    "ar_MapTo",
    "DecompileScript",
    "HasScriptCommand",
    "GetCommandOpcode",
    "ar_Generate",
    "ar_Init",
    "ar_DeepEquals",
    "ar_ForEach",
    "ar_Any",
    "ar_All",
    "GetWeaponRegenRate",
    "SetWeaponRegenRate",
    "ar_Unique",
    "CallFunctionCond",
    "CallWhen_OLD",
    "ForEachInList",
    "TernaryUDF",
    "ModUIFloat",
    "GetQuestObjectiveCount",
    "GetNthQuestObjective",
    "GetCurrentObjective",
    "PrintActiveTile",
    "SetCurrentQuest",
    "sv_Trim",
    "CallAfterSeconds",
    "CallForSeconds",
    "CallWhile",
    "CallWhen",
    "SetEditorID",
    "Assert",
    "DispatchEventAlt",
    "DumpEventHandlers",
    "GetEventHandlers",
    "GetSelfAlt_OLD",
    "SetEventHandlerAlt",
    "CreateFormList",
    "CallWhilePerSeconds",
    "CallAfterFrames",
    "GetSoldItemInvRef",
    "IsEventHandlerFirst",
    "IsEventHandlerLast",
    "GetHigherPriorityEventHandlers",
    "GetLowerPriorityEventHandlers",
    "ValidateRegex",
    "IntToBin",
    "NumToHex",
    "HasAmmoEquipped",
    "GetEquippedWeaponCanUseAmmo",
    "IsEquippedAmmoInList",
    "GetEquippedWeaponUsesAmmoList",
    "IsInventoryRef",
    "DebugPrintVar",
    "SetStringIniSetting",
    "GetHeadingAngleX",
    "GetWeaponCanUseAmmo",
    "SetAmmoConsumedPercent",
    "DisablePlayerControlsAlt",
    "EnablePlayerControlsAlt",
    "GetPlayerControlsDisabledAlt",
    "DisablePlayerControlsAltEx",
    "EnablePlayerControlsAltEx",
    "GetPlayerControlsDisabledAltEx",
    "CopyIRAlt",
    "CompileScript",
    "GetSelfAlt",
    "DumpDocs",
    "DumpCommandWikiDoc",
    "DumpCommandWikiDocs",
    "SetModelPath",
    "Ternary",
    "MatchesAnyOf",
    "ForEachAlt",
    "ar_Exists",
    "ar_Count",
    "ar_CountWhere",
    "EvaluateInventory",
    "ar_GetNth",
    "PluginVersion",
    "GetDoorSound",
    "FireChallenge",
    "GetDisabledKeys",
    "ReloadPluginConfig",
    "GetPressedKeys",
    "GetStringVariable",
    "SetStringVariable",
    "ar_Cat",
    "Jmp_If_True",
    "Jmp_If_False",
    "Jmp",
    "V3NormalizeEx",
    "V3CrossproductEx",
    "QFromEulerEx",
    "QFromAxisAngleEx",
    "QNormalizeEx",
    "QMultQuatQuatEx",
    "QMultQuatVector3Ex",
    "QInterpolateEx",
    "QToEulerEx",
    "GetUIFloatInherited",
    "ListGetSaveBakedObjectCount",
    "GetNumLevSaveBakedItems",
];

/// An xNVSE function's name by opcode.
pub fn name(opcode: u16) -> Option<&'static str> {
    let i = usize::from(opcode.checked_sub(FIRST_OPCODE)?);
    FUNCTIONS.get(i).copied()
}

/// An xNVSE function's opcode by name (case-insensitive).
pub fn opcode(name: &str) -> Option<u16> {
    FUNCTIONS
        .iter()
        .position(|n| n.eq_ignore_ascii_case(name))
        .map(|i| FIRST_OPCODE + i as u16)
}

/// How a call was found in a compiled script.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Found {
    /// A statement of its own (`SomeFunction arg`).
    Statement,
    /// Inside an `if`, `elseif` or `set` expression stored the game's way.
    Expression,
    /// Inside an expression stored xNVSE's way (the arguments of `Let`,
    /// `eval` and the other functions xNVSE parses itself): found by
    /// scanning those bytes for xNVSE's call token (`X` or `x`, the
    /// reference's number, the opcode, the arguments' length; xNVSE's
    /// `ScriptToken::ReadFrom`), so a stray byte pattern can be miscounted.
    NvseExpression,
}

/// Every function call (opcode) in a script's compiled bytes (`SCDA`), in
/// order, with how each was found. Game functions, xNVSE's and NVSE
/// plugins' alike: callers keep the ranges they want.
pub fn calls(compiled: &[u8]) -> Vec<(u16, Found)> {
    use crate::compiled::{expression, statements, Token};
    let mut out = Vec::new();
    for s in statements(compiled) {
        if s.code >= 0x1000 {
            out.push((s.code, Found::Statement));
            if s.code >= FIRST_OPCODE {
                scan_nvse(s.data, &mut out);
            }
            continue;
        }
        let start = match s.code {
            // if, elseif: the jump, then the expression's length.
            0x16 | 0x18 => 2,
            // set: the target (a reference's variable, or a local).
            0x15 if s.data.first() == Some(&b'r') => 6,
            0x15 => 3,
            _ => continue,
        };
        let Some(len) = s
            .data
            .get(start..start + 2)
            .map(|b| usize::from(u16::from_le_bytes([b[0], b[1]])))
        else {
            continue;
        };
        let Some(bytes) = s.data.get(start + 2..start + 2 + len) else {
            continue;
        };
        match expression(bytes) {
            Some(tokens) => {
                for t in tokens {
                    if let Token::Call(code, params) = t {
                        out.push((code, Found::Expression));
                        if code >= FIRST_OPCODE {
                            scan_nvse(params, &mut out);
                        }
                    }
                }
            }
            None => scan_nvse(bytes, &mut out),
        }
    }
    out
}

/// Whether an opcode is one a script can call: the game's table, xNVSE's,
/// or an NVSE plugin's range.
fn callable(opcode: u16) -> bool {
    let game = 0x1000..0x1000 + crate::functions::FUNCTIONS.len() as u16;
    game.contains(&opcode) || name(opcode).is_some() || opcode >= PLUGIN_OPCODES
}

/// xNVSE's call tokens in its own expression bytes.
fn scan_nvse(bytes: &[u8], out: &mut Vec<(u16, Found)>) {
    let u16_at = |i: usize| {
        bytes
            .get(i..i + 2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
    };
    let mut at = 0;
    while at < bytes.len() {
        if matches!(bytes[at], b'X' | b'x') {
            if let (Some(_reference), Some(opcode), Some(len)) =
                (u16_at(at + 1), u16_at(at + 3), u16_at(at + 5))
            {
                // The length counts from just after the opcode, so it
                // covers itself (two bytes) and the arguments.
                let len = usize::from(len);
                if callable(opcode) && len >= 2 && at + 5 + len <= bytes.len() {
                    out.push((opcode, Found::NvseExpression));
                    at += 7;
                    continue;
                }
            }
        }
        at += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_opcodes_xnvse_hard_codes() {
        // nvse/nvse/Compiler/Passes/Compiler.cpp, the same commit.
        for (op, n) in [
            (0x1539, "Let"),
            (0x153A, "eval"),
            (0x153B, "While"),
            (0x153C, "Loop"),
            (0x153D, "ForEach"),
            (0x153E, "Continue"),
            (0x153F, "Break"),
            (0x1545, "Call"),
            (0x1546, "SetFunctionValue"),
            (0x1548, "GetModLocalData"),
            (0x1549, "SetModLocalData"),
            (0x1557, "ar_Find"),
            (0x155F, "ar_BadNumericIndex"),
            (0x1567, "ar_List"),
            (0x1568, "ar_Map"),
            (0x166E, "Ternary"),
            (0x166F, "MatchesAnyOf"),
            (0x1670, "ForEachAlt"),
            (0x1671, "ar_Exists"),
            (0x1676, "PluginVersion"),
        ] {
            assert_eq!(name(op), Some(n), "{op:#x}");
            assert_eq!(opcode(n), Some(op));
        }
        assert_eq!(name(0x1400), Some("GetNVSEVersion"));
        assert_eq!(name(0x13FF), None);
        assert_eq!(name(FIRST_OPCODE + FUNCTIONS.len() as u16), None);
    }

    #[test]
    fn finds_calls_in_statements_expressions_and_nvse_expressions() {
        let mut bytes = vec![
            0x1D, 0x00, 0x00, 0x00, // ScriptName
            0x10, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Begin GameMode
            0x00, 0x14, 0x02, 0x00, 0x00, 0x00, // GetNVSEVersion as a statement
        ];
        // if GetNVSERevision, stored the game's way: the jump, the
        // expression's length, then ` X <opcode> <length 2> <count 0>`.
        let expr = [b' ', b'X', 0x01, 0x14, 0x02, 0x00, 0x00, 0x00];
        bytes.extend([0x16, 0x00]);
        bytes.extend(((4 + expr.len()) as u16).to_le_bytes());
        bytes.extend([0x00, 0x00]);
        bytes.extend((expr.len() as u16).to_le_bytes());
        bytes.extend(expr);
        // Let, whose argument holds xNVSE's call token for ar_List:
        // reference 0, the opcode, a length of 2 (only its own bytes).
        let nvse = [0x01, 0x01, b'X', 0x00, 0x00, 0x67, 0x15, 0x02, 0x00];
        bytes.extend([0x39, 0x15]);
        bytes.extend((nvse.len() as u16).to_le_bytes());
        bytes.extend(nvse);
        bytes.extend([0x11, 0x00, 0x00, 0x00]); // End
        assert_eq!(
            calls(&bytes),
            [
                (0x1400, Found::Statement),
                (0x1401, Found::Expression),
                (0x1539, Found::Statement),
                (0x1567, Found::NvseExpression),
            ]
        );
    }

    #[test]
    fn ignores_byte_patterns_that_are_not_calls() {
        // An `X` whose opcode is in no table, and one whose length runs
        // past the end.
        let mut out = Vec::new();
        scan_nvse(&[b'X', 0, 0, 0x00, 0x13, 2, 0], &mut out);
        scan_nvse(&[b'X', 0, 0, 0x00, 0x14, 9, 0], &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn scripts_calling_them_do_not_parse_from_source() {
        // nv-rs runs scripts from their source; the script extender's
        // functions aren't in the game's table, so such a script fails.
        let source = "scn Uses
begin GameMode
GetNVSEVersion
end
";
        assert!(crate::parse(source).is_err());
        assert!(crate::parse(
            "scn Plain
begin GameMode
GetStage VMS16
end
"
        )
        .is_ok());
    }
}
