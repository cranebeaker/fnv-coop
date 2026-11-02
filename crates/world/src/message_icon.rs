//! The pictures beside the HUD's corner messages (FalloutNV.exe 1.4.0.525).
//!
//! The game's `QueueUIMessage` (`007052f0` → the HUD's `00775380`) takes
//! the text, a type, a picture, a sound, how long and whether it goes
//! first. With no picture the type picks a Vault Boy: 0 neutral, 1 very
//! happy, 2 sad, 3 in pain. Callers pass their own: a script's
//! `ShowMessage` of a message that isn't a box passes the message's icon
//! (`005b4630`: the record's `INAM`, a `MICN` record's `ICON`, else none:
//! neutral); the casino games their refusals with the surprised Vault Boy,
//! chips lost with the sad one, a ban with the very happy one; the
//! lockpicking menu the surprised one.

use esm::{FormId, FourCC, LoadOrder};

/// The folder the message pictures are in.
const FOLDER: &str = "Interface\\Icons\\Message Icons\\";

/// The four Vault Boys `00775380` picks by type, and the surprised one
/// callers pass.
pub const NEUTRAL: &str = "Interface\\Icons\\Message Icons\\glow_message_vaultboy_neutral.dds";
pub const VERY_HAPPY: &str =
    "Interface\\Icons\\Message Icons\\glow_message_vaultboy_very_happy.dds";
pub const SAD: &str = "Interface\\Icons\\Message Icons\\glow_message_vaultboy_sad.dds";
pub const IN_PAIN: &str = "Interface\\Icons\\Message Icons\\glow_message_vaultboy_in_pain.dds";
pub const SURPRISED: &str = "Interface\\Icons\\Message Icons\\glow_message_vaultboy_surprised.dds";
/// The gift box of the "added" notice (`004821a0`, `0101c140`).
pub const GIFT_BOX: &str = "Interface\\Icons\\Message Icons\\glow_message_giftbox.dds";

/// The other pictures callers pass.
pub const PADLOCK: &str = "Interface\\Icons\\Message Icons\\glow_message_padlock.dds";
pub const KEY: &str = "Interface\\Icons\\Message Icons\\glow_message_key.dds";
pub const MAP: &str = "Interface\\Icons\\Message Icons\\glow_message_map.dds";
pub const RADIO_TOWER: &str = "Interface\\Icons\\Message Icons\\glow_message_radio_tower.dds";
pub const THINKING: &str = "Interface\\Icons\\Message Icons\\glow_message_vaultboy_thinking.dds";
/// A terminal the player can't hack, activated (`00501310`).
pub const ANGRY: &str = "Interface\\Icons\\Message Icons\\glow_message_vaultboy_angry.dds";

/// The picture the game's own code passes with a message showing a text
/// setting: each `QueueUIMessage` call (`007052f0`'s callers) read with
/// the setting it shows and the picture pushed for it, or the Vault Boy
/// its type picks when it pushes none (`00775380`: 1 very happy, 2 sad;
/// 0 the neutral one, listed where the game's code shows it). (Settings
/// shown with different pictures by different callers aren't here.)
pub fn for_setting(name: &str) -> Option<&'static str> {
    const TABLE: &[(&str, &str)] = &[
        // Items (`007284f0`, `004c37d0`, `0088c830`, `008929a0`,
        // `00780140`, `00781ba0`, `005d0060`, `005d0300`, `005c1d40`).
        ("sAddItemtoInventory", GIFT_BOX),
        ("sCantRemoveWornItem", SAD),
        ("sCantEquipBrokenItem", SAD),
        ("sCantEquipGeneric", SAD),
        ("sCantEquipPowerArmor", SAD),
        ("sNoEatQuestItem", SAD),
        ("sAnimationCanNotUnequip", SAD),
        ("sCantHotkeyItem", SAD),
        ("sCantHotkeyBrokenItem", SAD),
        // Type 2 (`00780140`: ITEMS' refusals).
        ("sDropQuestItemWarning", SAD),
        ("sDropEquippedItemWarning", SAD),
        ("sNoJumpWarning", SAD),
        ("sNotEnoughRoomWarning", SAD),
        // Books (`00515040`: type 1, type 0) and chems (`00824e70` type
        // 1, `00823c40` type 0).
        ("sSkillIncreasedNum", VERY_HAPPY),
        ("sCanNotReadBook", NEUTRAL),
        ("sChemsAddicted", VERY_HAPPY),
        ("sChemsWithdrawal", NEUTRAL),
        ("sChemsWornOff", NEUTRAL),
        // Condition (`00891360`: type 2).
        ("sWeaponLowCond", SAD),
        ("sArmorLowCond", SAD),
        ("sWeaponBreak", SAD),
        ("sEquipItemOnPlayer", NEUTRAL),
        ("sUnequipItemOnPlayer", NEUTRAL),
        ("sAddItemtoSpellList", VERY_HAPPY),
        // Locks and doors (`00516dc0`, `005180b0`, `0078db00`, `0078eb50`,
        // `00790330`, `00573170`).
        ("sImpossibleLock", PADLOCK),
        ("sOpenWithKey", KEY),
        ("sLockpickSkillTooLow", PADLOCK),
        ("sOutOfLockpicks", SURPRISED),
        ("sLockBroken", SURPRISED),
        ("sRandomDoorTeleportFailureMessage", SAD),
        ("sRemoteActivation", THINKING),
        // People (`005fa330`, `00607990`, `0089d900`, `0089a760`).
        ("sNoTalkUnConscious", SURPRISED),
        ("sNoTalkFleeing", SAD),
        ("sNoPickPocketAgain", SAD),
        // Type 2: caught (`0075e0b0`), a companion carrying too much
        // (`0075dc80`); type 0: a calmed person (`00607990`, `00756980`).
        ("sPickpocketFail", SAD),
        ("sTeammateOverencumbered", SAD),
        ("sActivateNPCCalmed", NEUTRAL),
        ("sEssentialCharacterDown", SURPRISED),
        ("sCriticalStrike", VERY_HAPPY),
        ("sSneakAttackCriticalStrike", VERY_HAPPY),
        ("sOverEncumbered", SURPRISED),
        // Sleeping and waiting (`005095b0`, `00969fa0`).
        ("sNoSleepInOwnedBed", SAD),
        ("sNoSleepTrespass", SAD),
        ("sNoSleepHostileActorsNear", SAD),
        ("sNoSleepInRadiation", SAD),
        ("sNoSleepTakingHealthDamage", SAD),
        ("sNoWaitTrespass", SAD),
        ("sNoWaitWhileAlarmSounding", SAD),
        ("sNoWaitUnderWater", SAD),
        ("sNoWaitHostilActorsNear", SAD),
        ("sNoWaitInAir", SAD),
        ("sNoWaitInCell", SAD),
        ("sNoWaitInRadiation", SAD),
        ("sNoWaitTakingHealthDamage", SAD),
        // Travel (`0093d660`, `0094dbe0`, `005c8620`, `00833d00`).
        ("sNoFastTravelHostileActorsNear", SAD),
        ("sNoFastTravelAlarm", SAD),
        ("sFastTravelNoTravelHealthDamage", SAD),
        ("sNoFastTravelScriptBlock", SAD),
        ("sNoFastTravelInAir", SAD),
        ("sNoFastTravelCell", SAD),
        // Type 0 (`0093d660`) and type 2 (the map, `00796fd0`).
        ("sNoFastTravelOverencumbered", NEUTRAL),
        ("sNoFastTravelUndiscovered", SAD),
        ("sPlayerLeavingBorderRegion", SAD),
        ("sMapMarkerAdded", MAP),
        ("sRadioStationDiscovered", RADIO_TOWER),
        // Saving (`0070c4a0`, `008503b0`, `00856ca0`, `008509a0`,
        // `008509f0`).
        ("sCantQuickSave", SAD),
        ("sCantQuickLoad", SAD),
        ("sCantSaveNow", SAD),
        ("sSaveFailed", SAD),
        ("sQuickSaving", NEUTRAL),
        ("sQuickLoading", NEUTRAL),
        // Hardcore needs and radiation (`008c5350`, `008c5610`, `008c5890`,
        // `008c5b10`).
        ("sRadiationIncrease", IN_PAIN),
        ("sRadiationDecrease", VERY_HAPPY),
        ("sRadiationNotSick", VERY_HAPPY),
        ("sRadiationSick", IN_PAIN),
        ("sDehydrationIncrease", IN_PAIN),
        ("sDehydrationDecrease", VERY_HAPPY),
        ("sDehydrationNotSick", VERY_HAPPY),
        ("sHungerIncrease", IN_PAIN),
        ("sHungerDecrease", VERY_HAPPY),
        ("sHungerNotSick", VERY_HAPPY),
        ("sSleepDeprevationIncrease", IN_PAIN),
        ("sSleepDeprevationDecrease", VERY_HAPPY),
        ("sSleepDeprevationNotSick", VERY_HAPPY),
        // Caravan (`00741060`), the casinos (`00733630`, `007bbe20`,
        // `007c0a40`: refusals).
        ("sCardCountText", SURPRISED),
        ("sGamblingBrokeText", SURPRISED),
        ("sGamblingMinWinText", SURPRISED),
        ("sBlackjackAntiCheatText", SURPRISED),
        ("sRouletteAntiCheatText", SURPRISED),
        ("sSlotAntiCheatText", SURPRISED),
    ];
    TABLE
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, icon)| *icon)
}

/// A picture the game reads from a text setting (the reputation messages'
/// `sRep…Icon`, karma's `sKarma…Image`): the data's, else the exe's.
pub fn from_setting(order: &LoadOrder, name: &str, exe: &str) -> String {
    crate::scripting::game_setting_text(order, name).unwrap_or_else(|| exe.to_string())
}

/// The picture for a message of a type with no picture of its own
/// (`00775380`'s switch; another type leaves the picture empty).
pub fn by_type(kind: u32) -> Option<&'static str> {
    [NEUTRAL, VERY_HAPPY, SAD, IN_PAIN]
        .get(kind as usize)
        .copied()
}

/// A message record's picture (`BGSMessage`'s icon: `INAM`, a menu icon
/// record whose `ICON` is the picture), if it has one.
pub fn of_message(order: &LoadOrder, message: FormId) -> Option<String> {
    let rr = order.get(message)?;
    let record = rr.record().ok()?;
    let inam = record.get(FourCC::new(b"INAM"))?;
    let raw = u32::from_le_bytes(inam.data.get(..4)?.try_into().ok()?);
    if raw == 0 {
        return None;
    }
    let icon = order.get(rr.plugin.to_global(FormId(raw)))?.record().ok()?;
    let path = icon.get(FourCC::new(b"ICON"))?.zstring();
    (!path.trim().is_empty()).then_some(path)
}

/// Whether a picture is one of the message pictures' folder (for tests and
/// callers checking what they were given).
pub fn is_message_icon(path: &str) -> bool {
    path.len() > FOLDER.len() && path[..FOLDER.len()].eq_ignore_ascii_case(FOLDER)
}

#[cfg(test)]
mod tests {
    use super::*;
    use testdata::{group, record, sub, zstr};

    fn order() -> LoadOrder {
        let mut header = 1.34f32.to_le_bytes().to_vec();
        header.extend([0; 8]);
        let mut bytes = record(b"TES4", 0, &sub(b"HEDR", &header));
        let mut micn = sub(b"EDID", &zstr("MenuIconVaultBoyInPain"));
        micn.extend(sub(b"ICON", &zstr(IN_PAIN)));
        bytes.extend(group(*b"MICN", 0, &record(b"MICN", 0x60C28, &micn)));
        let mesg = |id: u32, icon: u32| {
            let mut d = sub(b"EDID", &zstr("M"));
            d.extend(sub(b"DESC", &zstr("Text")));
            d.extend(sub(b"INAM", &icon.to_le_bytes()));
            d.extend(sub(b"DNAM", &0u32.to_le_bytes()));
            record(b"MESG", id, &d)
        };
        let mut g = mesg(0x190, 0x60C28);
        g.extend(mesg(0x191, 0));
        bytes.extend(group(*b"MESG", 0, &g));
        let plugin = esm::Plugin::from_bytes(bytes).unwrap();
        LoadOrder::single("FalloutNV.esm", None, plugin).unwrap()
    }

    #[test]
    fn a_message_shows_its_menu_icon() {
        let order = order();
        assert_eq!(of_message(&order, FormId(0x190)).as_deref(), Some(IN_PAIN));
        assert_eq!(of_message(&order, FormId(0x191)), None);
        assert_eq!(of_message(&order, FormId(0x192)), None);
    }

    #[test]
    fn the_games_messages_have_their_pictures() {
        assert_eq!(for_setting("sCantEquipBrokenItem"), Some(SAD));
        assert_eq!(for_setting("simpossiblelock"), Some(PADLOCK));
        assert_eq!(for_setting("sRadiationIncrease"), Some(IN_PAIN));
        assert_eq!(for_setting("sHungerNotSick"), Some(VERY_HAPPY));
        assert_eq!(for_setting("sGamblingBrokeText"), Some(SURPRISED));
        // By their type: sad, very happy.
        assert_eq!(for_setting("sWeaponBreak"), Some(SAD));
        assert_eq!(for_setting("sPickpocketFail"), Some(SAD));
        assert_eq!(for_setting("sSkillIncreasedNum"), Some(VERY_HAPPY));
        assert_eq!(for_setting("sNoFastTravelOverencumbered"), Some(NEUTRAL));
        // Shown with different pictures by different callers.
        assert_eq!(for_setting("sHackIneligible"), None);
        assert!(is_message_icon(KEY) && is_message_icon(RADIO_TOWER));
    }

    #[test]
    fn types_pick_the_vault_boys() {
        assert_eq!(by_type(0), Some(NEUTRAL));
        assert_eq!(by_type(2), Some(SAD));
        assert_eq!(by_type(4), None);
        assert!(is_message_icon(SURPRISED));
        assert!(!is_message_icon("Interface\\Icons\\x.dds"));
    }
}
