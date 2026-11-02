//! `fallout/ai/falloutcharacterprogression.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The player character pointer.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The integer setting whose value (`0043d4d0`) is the level cap.
const MAX_LEVEL_SETTING: u32 = 0x011d_0c60;
/// `"UILevelUp"`.
const LEVEL_UP_SOUND: u32 = 0x0106_f354;

/// `00559450` (`thiscall`): the word at `this` (here: the address of the
/// ready-to-level-up byte).
const WORD_AT: u32 = 0x0055_9450;
/// `0087f9f0` (`thiscall` on the player): the player's level (`AX`).
const PLAYER_LEVEL: u32 = 0x0087_f9f0;
/// An integer setting's value address (`0043d4d0`, `thiscall`).
const INT_SETTING_VALUE: u32 = 0x0043_d4d0;
/// `BGSEntryPoint::HandleEntryPoint` (engine map), `cdecl`: entry point,
/// perk owner, address of the value.
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// `BSAudio::QInstance` (engine map), no stack words: the audio singleton.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `00ad7fa0` (`thiscall`, `RET 8`; the engine map names it
/// `BSAudio::Precache`): plays the named sound with the flags word.
const PLAY_SOUND: u32 = 0x00ad_7fa0;
/// `004e4430` (`cdecl`, one `float`): the experience after the game's
/// adjustment, in `ST0`.
const ADJUST_EXPERIENCE: u32 = 0x004e_4430;
/// `_ftol2_sse` (`00ec62c0`): truncates the value in `ST0` (an `f64` here).
const FTOL: u32 = 0x00ec_62c0;
/// `GamePlayFormulas::GetRequiredExperiencePoints` (engine map), `cdecl`:
/// level.
const REQUIRED_EXPERIENCE: u32 = 0x0064_8b50;

// Translated from 008d51f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharacterProgression::QReadyToLevelUp` (Xbox PDB): the byte at the
/// address `00559450` gives for `this`.
pub fn character_progression_q_ready_to_level_up(e: &mut Engine, this: Ptr) -> u8 {
    let address = e.call(WORD_AT, &args![this]).u32();
    e.mem.u8(address)
}

// Translated from 008d5100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharacterProgression::RewardExperience` (Xbox PDB; `ECX` not read,
/// `RET 4`). Below the level cap (the player's level from `0087f9f0` as a
/// 16-bit value, compared signed with the setting at `011d0c60`): the
/// experience as a float runs through entry point 9 (`005e58f0`), the
/// `"UILevelUp"` sound plays, the adjusted amount (`004e4430`, truncated)
/// is cut so that the player's experience (actor value 0x18, through the
/// value owner at +0xa4, slot +8) does not pass the experience the cap level
/// needs (`00648b50`), and the player's slot +0x3a8 adds it to actor value
/// 0x18.
pub fn character_progression_reward_experience(e: &mut Engine, _unused_0: Ptr, experience: i32) {
    let player = e.global::<u32>(PLAYER_POINTER);
    let level = e.call(PLAYER_LEVEL, &args![player]).u16();
    let cap_address = e.call(INT_SETTING_VALUE, &args![MAX_LEVEL_SETTING]).u32();
    if i32::from(level) >= e.mem.i32(cap_address) {
        return;
    }
    let amount_bits = x87_store32(f64::from(experience));
    let amount_bits = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), amount_bits);
        let player = e.global::<u32>(PLAYER_POINTER);
        e.call(HANDLE_ENTRY_POINT, &args![9u32, player, slot]);
        e.mem.u32(slot.addr())
    });
    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
    e.call(PLAY_SOUND, &args![audio, LEVEL_UP_SOUND, 0x121u32]);
    let adjusted = e
        .call(ADJUST_EXPERIENCE, &args![x87_quiet_bits(amount_bits)])
        .f64();
    let mut reward = e.call(FTOL, &args![adjusted]).i32();
    let player = e.global::<u32>(PLAYER_POINTER);
    let current = e.vcall(player.wrapping_add(0xa4), 8, &args![0x18u32]).i32();
    let total = current.wrapping_add(reward);
    let cap_address = e.call(INT_SETTING_VALUE, &args![MAX_LEVEL_SETTING]).u32();
    let cap = e.mem.u32(cap_address);
    let required = e.call(REQUIRED_EXPERIENCE, &args![cap]).i32();
    if total > required {
        reward = reward.wrapping_sub(total.wrapping_sub(required));
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    e.vcall(player, 0x3a8, &args![0x18u32, reward, 0u32]);
}

/// `004181e0` (`thiscall`): the reference's base object.
const GET_BASE_OBJECT: u32 = 0x0041_81e0;
/// `0047dfe0` (`thiscall` on the base object's +0x30 component, one word:
/// a level; no Xbox PDB name).
const BASE_DATA_SET_LEVEL_0047DFE0: u32 = 0x0047_dfe0;
/// `Interface::CreateLevelUpMenu` (engine map; no stack words).
const CREATE_LEVEL_UP_MENU: u32 = 0x0070_6270;
/// `008d55e0` (`thiscall` on the progression data, one word: the next
/// level; a byte; no Xbox PDB name).
const DATA_TEST_008D55E0: u32 = 0x008d_55e0;
/// `004614e0` (`thiscall` on the object at `011c3f2c`; no Xbox PDB name).
const DATA_HANDLER_CALL_004614E0: u32 = 0x0046_14e0;
/// The object [`fn_008d5270`] hands to `004614e0` (a pointer global).
const DATA_HANDLER_POINTER: u32 = 0x011c_3f2c;
/// `00975ea0` (`thiscall` on the object at `011e0e80`; no Xbox PDB name).
const PROCESS_LISTS_CALL_00975EA0: u32 = 0x0097_5ea0;
/// The object at `011e0e80` (`processlists.cpp` code runs on it).
const PROCESS_LISTS_OBJECT: u32 = 0x011e_0e80;
/// `00a25920` (no stack words; a pointer, possibly null; no Xbox PDB name).
const MENU_POINTER_00A25920: u32 = 0x00a2_5920;
/// `007d0bb0` (`thiscall` on what `00a25920` gives; `startmenu.cpp`).
const START_MENU_CALL_007D0BB0: u32 = 0x007d_0bb0;
/// `00454af0` (`thiscall` on the object at `011e0a30`; a byte; `tes.cpp`).
const TES_TEST_00454AF0: u32 = 0x0045_4af0;
/// The object at `011e0a30` that `00454af0` runs on.
const TES_OBJECT_011E0A30: u32 = 0x011e_0a30;
/// `PlayerCharacter::ExportProgressData` (engine map; `thiscall`, one
/// word).
const EXPORT_PROGRESS_DATA: u32 = 0x0094_ed40;
/// `00464e10` (`thiscall` on the player; no Xbox PDB name): the object
/// whose `00559450` word is the progression data.
const PLAYER_PROGRESSION_00464E10: u32 = 0x0046_4e10;
/// `StatsMenu::UpdatePlayerName` and `StatsMenu::UpdatePlayerTitle`
/// (engine map; no stack words).
const UPDATE_PLAYER_NAME: u32 = 0x007d_f4e0;
const UPDATE_PLAYER_TITLE: u32 = 0x007d_f5d0;

// Translated from 008d5210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharacterProgression::BeginLevelUp` (Xbox PDB): when the
/// ready-to-level-up byte (at the address `00559450` gives) is set, plays
/// `"UILevelUp"` (flags 0x121), hands the player's level + 1 (the 16-bit
/// level zero-extended) to `0047dfe0` on the base object's +0x30 component
/// and opens the level-up menu (`Interface::CreateLevelUpMenu`).
pub fn character_progression_begin_level_up(e: &mut Engine, this: Ptr) {
    let address = e.call(WORD_AT, &args![this]).u32();
    if e.mem.u8(address) == 0 {
        return;
    }
    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
    e.call(PLAY_SOUND, &args![audio, LEVEL_UP_SOUND, 0x121u32]);
    let player = e.global::<u32>(PLAYER_POINTER);
    let level = e.call(PLAYER_LEVEL, &args![player]).u16();
    let next = u32::from(level) + 1;
    let player = e.global::<u32>(PLAYER_POINTER);
    let base = e.call(GET_BASE_OBJECT, &args![player]).u32();
    e.call(
        BASE_DATA_SET_LEVEL_0047DFE0,
        &args![base.wrapping_add(0x30), next],
    );
    e.call(CREATE_LEVEL_UP_MENU, &[]);
}

// Translated from 008d5270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The level-up check after a level change (`thiscall`; no Xbox PDB name):
/// unless `008d55e0(data, level + 1)` answers true (the data at the
/// address `00559450` gives, the 16-bit level zero-extended), clears the
/// ready-to-level-up byte, runs `004614e0` on the object at `011c3f2c` and
/// `00975ea0` on `011e0e80`, `007d0bb0` on what `00a25920` gives (when not
/// null), and, when `00454af0(011e0a30)` answers true,
/// `PlayerCharacter::ExportProgressData(player, 0)`.
pub fn fn_008d5270(e: &mut Engine, this: Ptr) {
    let player = e.global::<u32>(PLAYER_POINTER);
    let level = e.call(PLAYER_LEVEL, &args![player]).u16();
    let next = u32::from(level) + 1;
    let data = e.call(WORD_AT, &args![this]).u32();
    if e.call(DATA_TEST_008D55E0, &args![data, next]).bool() {
        return;
    }
    let address = e.call(WORD_AT, &args![this]).u32();
    e.mem.set_u8(address, 0);
    let handler = e.global::<u32>(DATA_HANDLER_POINTER);
    e.call(DATA_HANDLER_CALL_004614E0, &args![handler]);
    e.call(PROCESS_LISTS_CALL_00975EA0, &args![PROCESS_LISTS_OBJECT]);
    if e.call(MENU_POINTER_00A25920, &[]).u32() != 0 {
        let menu = e.call(MENU_POINTER_00A25920, &[]).u32();
        e.call(START_MENU_CALL_007D0BB0, &args![menu]);
    }
    if e.call(TES_TEST_00454AF0, &args![TES_OBJECT_011E0A30])
        .bool()
    {
        let player = e.global::<u32>(PLAYER_POINTER);
        e.call(EXPORT_PROGRESS_DATA, &args![player, 0u32]);
    }
}

// Translated from 008d5300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharacterProgression::ForceLevel` (Xbox PDB; `RET 4`): nothing when
/// `level` is the player's level (the 16-bit level zero-extended, compared
/// with all 32 bits). Otherwise, with the progression data (the
/// `00559450` word of `00464e10(player)`): the player's experience (actor
/// value 0x18, value owner +0xA4, slot +8) is moved to the experience
/// `level - 1` needs (`00648b50`) through the player's slot +0x3A8 (the
/// difference, with the data's byte +8 set for the call), the data's byte
/// +0 is cleared and its +4 becomes the experience `level + 1` needs; the
/// level's low 16 bits go to `0047dfe0` on the base object's +0x30
/// component, then [`fn_008d5270`] and `StatsMenu::UpdatePlayerName` /
/// `UpdatePlayerTitle`.
pub fn character_progression_force_level(e: &mut Engine, this: Ptr, level: u32) {
    let player = e.global::<u32>(PLAYER_POINTER);
    let current_level = e.call(PLAYER_LEVEL, &args![player]).u16();
    if level == u32::from(current_level) {
        return;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    let holder = e.call(PLAYER_PROGRESSION_00464E10, &args![player]).u32();
    let data = e.call(WORD_AT, &args![holder]).u32();
    let player = e.global::<u32>(PLAYER_POINTER);
    let experience = e.vcall(player.wrapping_add(0xa4), 8, &args![0x18u32]).u32();
    let required = e
        .call(REQUIRED_EXPERIENCE, &args![level.wrapping_sub(1)])
        .u32();
    let difference = required.wrapping_sub(experience);
    e.mem.set_u8(data.wrapping_add(8), 1);
    let player = e.global::<u32>(PLAYER_POINTER);
    e.vcall(player, 0x3a8, &args![0x18u32, difference, 0u32]);
    e.mem.set_u8(data.wrapping_add(8), 0);
    e.mem.set_u8(data, 0);
    let next = e
        .call(REQUIRED_EXPERIENCE, &args![level.wrapping_add(1)])
        .u32();
    e.mem.set_u32(data.wrapping_add(4), next);
    let player = e.global::<u32>(PLAYER_POINTER);
    let base = e.call(GET_BASE_OBJECT, &args![player]).u32();
    e.call(
        BASE_DATA_SET_LEVEL_0047DFE0,
        &args![base.wrapping_add(0x30), level & 0xffff],
    );
    fn_008d5270(e, this);
    e.call(UPDATE_PLAYER_NAME, &[]);
    e.call(UPDATE_PLAYER_TITLE, &[]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_5210, character_progression_begin_level_up(Ptr)),
        entry!(0x008d_5270, fn_008d5270(Ptr)),
        entry!(0x008d_5300, character_progression_force_level(Ptr, u32)),
        entry!(0x008d_51f0, character_progression_q_ready_to_level_up(Ptr) -> u8),
        entry!(
            0x008d_5100,
            character_progression_reward_experience(Ptr, i32)
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn q_ready_to_level_up_reads_the_byte() {
        let mut e = Engine::new();
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        e.register_double(WORD_AT, move |_, _| word(flag));
        assert_eq!(e.call(0x008d_51f0, &args![0x5000u32]).u8(), 1);
    }

    /// The player with a value owner at +0xa4 whose slot +8 answers
    /// `current`, and its slot +0x3a8 recorded.
    fn setup(e: &mut Engine, level: u32, cap: u32, current: u32) {
        let player = e.mem.alloc(0x100);
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER_POINTER, player);
        e.put_vtable(0x0300_0000, &[0; 0xeb]);
        e.mem.set_u32(0x0300_0000 + 0x3a8, 0x0300_2000);
        e.mem.set_u32(player, 0x0300_0000);
        e.put_vtable(0x0300_1000, &[0, 0, 0x0300_3000]);
        e.mem.set_u32(player + 0xa4, 0x0300_1000);
        e.register(0x0300_2000, |_, _| Ret::default());
        e.register_double(0x0300_3000, move |_, _| word(current));
        e.register_double(PLAYER_LEVEL, move |_, _| word(0xffff_0000 | level));
        let cap_address = e.mem.alloc(4);
        e.mem.set_u32(cap_address, cap);
        e.register_double(INT_SETTING_VALUE, move |_, _| word(cap_address));
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            // The perk doubles the value.
            let value = e.mem.f32(a[2]);
            e.mem.set_f32(a[2], value * 2.0);
            Ret::default()
        });
        e.register(AUDIO_INSTANCE, |_, _| word(0x0400_0000));
        e.register(PLAY_SOUND, |_, _| Ret::default());
        e.register(ADJUST_EXPERIENCE, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) + 0.75,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word(value as i32 as u32)
        });
        e.register(REQUIRED_EXPERIENCE, |_, a| word(a[0] * 1000));
    }

    #[test]
    fn reward_experience_adds_the_adjusted_amount() {
        let mut e = Engine::new();
        setup(&mut e, 3, 30, 100);
        e.call_log = Some(vec![]);
        e.call(0x008d_5100, &args![0u32, 25i32]);
        assert_eq!(
            calls_to(&e, PLAY_SOUND),
            vec![vec![0x0400_0000, LEVEL_UP_SOUND, 0x121]]
        );
        assert_eq!(calls_to(&e, HANDLE_ENTRY_POINT)[0][0], 9);
        assert_eq!(calls_to(&e, ADJUST_EXPERIENCE), vec![vec![50f32.to_bits()]]);
        let player = e.global::<u32>(PLAYER_POINTER);
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![player, 0x18, 50, 0]]);
        assert_eq!(calls_to(&e, REQUIRED_EXPERIENCE), vec![vec![30]]);
    }

    #[test]
    fn reward_experience_stops_at_the_cap_experience() {
        let mut e = Engine::new();
        setup(&mut e, 3, 30, 29_980);
        e.call_log = Some(vec![]);
        e.call(0x008d_5100, &args![0u32, 25i32]);
        let player = e.global::<u32>(PLAYER_POINTER);
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![player, 0x18, 20, 0]]);
    }

    #[test]
    fn reward_experience_does_nothing_at_the_level_cap() {
        let mut e = Engine::new();
        setup(&mut e, 30, 30, 0);
        e.call_log = Some(vec![]);
        e.call(0x008d_5100, &args![0u32, 25i32]);
        assert!(calls_to(&e, HANDLE_ENTRY_POINT).is_empty());
        assert!(calls_to(&e, 0x0300_2000).is_empty());
    }
}

#[cfg(test)]
mod n026_tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// The player (level `level` with garbage above bit 15) with a value
    /// owner at +0xA4 whose slot +8 answers `experience`, its slot +0x3A8
    /// recorded at 0x0300_2000; the progression data block is returned.
    fn setup(e: &mut Engine, level: u32, experience: u32) -> (u32, u32) {
        let player = e.mem.alloc(0x100);
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER_POINTER, player);
        e.put_vtable(0x0300_0000, &[0; 0xeb]);
        e.mem.set_u32(0x0300_0000 + 0x3a8, 0x0300_2000);
        e.mem.set_u32(player, 0x0300_0000);
        e.put_vtable(0x0300_1000, &[0, 0, 0x0300_3000]);
        e.mem.set_u32(player + 0xa4, 0x0300_1000);
        let data = e.mem.alloc(0x10);
        e.register_double(0x0300_2000, move |e, a| {
            // The data's byte +8 is set while the experience changes.
            assert_eq!(e.mem.u8(data + 8), 1);
            assert_eq!(a[1], 0x18);
            Ret::default()
        });
        e.register_double(0x0300_3000, move |_, _| experience.into_ret());
        e.register_double(PLAYER_LEVEL, move |_, _| (0xabcd_0000 | level).into_ret());
        e.register_double(WORD_AT, move |_, _| data.into_ret());
        e.register(AUDIO_INSTANCE, |_, _| 0x0400_0000u32.into_ret());
        e.register(GET_BASE_OBJECT, |_, _| 0x0500_0000u32.into_ret());
        e.register(REQUIRED_EXPERIENCE, |_, a| {
            a[0].wrapping_mul(1000).into_ret()
        });
        e.register(PLAYER_PROGRESSION_00464E10, |_, _| {
            0x0600_0000u32.into_ret()
        });
        quiet(
            e,
            &[
                PLAY_SOUND,
                BASE_DATA_SET_LEVEL_0047DFE0,
                CREATE_LEVEL_UP_MENU,
                DATA_HANDLER_CALL_004614E0,
                PROCESS_LISTS_CALL_00975EA0,
                START_MENU_CALL_007D0BB0,
                EXPORT_PROGRESS_DATA,
                UPDATE_PLAYER_NAME,
                UPDATE_PLAYER_TITLE,
            ],
        );
        e.map(DATA_HANDLER_POINTER & !0xfff, 0x1000);
        e.set_global(DATA_HANDLER_POINTER, 0x0700_0000u32);
        (player, data)
    }

    #[test]
    fn begin_level_up_needs_the_ready_byte() {
        let mut e = Engine::new();
        let (_, data) = setup(&mut e, 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x008d_5210, &args![0x100u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, PLAY_SOUND).is_empty());
        assert!(calls(&log, CREATE_LEVEL_UP_MENU).is_empty());

        e.mem.set_u8(data, 1);
        e.call_log = Some(vec![]);
        e.call(0x008d_5210, &args![0x100u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, PLAY_SOUND),
            vec![vec![0x0400_0000, LEVEL_UP_SOUND, 0x121]]
        );
        assert_eq!(
            calls(&log, BASE_DATA_SET_LEVEL_0047DFE0),
            vec![vec![0x0500_0030, 5]]
        );
        assert_eq!(calls(&log, CREATE_LEVEL_UP_MENU).len(), 1);
    }

    fn level_check(passes: bool, menu: u32, export: bool) -> (u32, Vec<(u32, Vec<u32>)>) {
        let mut e = Engine::new();
        let (player, data) = setup(&mut e, 0xffff, 0);
        e.mem.set_u8(data, 1);
        e.register_double(DATA_TEST_008D55E0, move |_, a| {
            assert_eq!(a[1], 0x1_0000);
            u32::from(passes).into_ret()
        });
        e.register_double(MENU_POINTER_00A25920, move |_, _| menu.into_ret());
        e.register_double(TES_TEST_00454AF0, move |_, _| {
            (0x100 | u32::from(export)).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x008d_5270, &args![0x100u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, DATA_TEST_008D55E0)[0][0], data);
        assert_eq!(e.mem.u8(data), u8::from(passes));
        (player, log)
    }

    #[test]
    fn level_check_clears_the_byte_and_refreshes() {
        let (_, log) = level_check(true, 0x0800_0000, true);
        assert!(calls(&log, DATA_HANDLER_CALL_004614E0).is_empty());

        let (player, log) = level_check(false, 0x0800_0000, true);
        assert_eq!(
            calls(&log, DATA_HANDLER_CALL_004614E0),
            vec![vec![0x0700_0000]]
        );
        assert_eq!(
            calls(&log, PROCESS_LISTS_CALL_00975EA0),
            vec![vec![PROCESS_LISTS_OBJECT]]
        );
        assert_eq!(
            calls(&log, START_MENU_CALL_007D0BB0),
            vec![vec![0x0800_0000]]
        );
        assert_eq!(
            calls(&log, TES_TEST_00454AF0),
            vec![vec![TES_OBJECT_011E0A30]]
        );
        assert_eq!(calls(&log, EXPORT_PROGRESS_DATA), vec![vec![player, 0]]);

        let (_, log) = level_check(false, 0, false);
        assert!(calls(&log, START_MENU_CALL_007D0BB0).is_empty());
        assert!(calls(&log, EXPORT_PROGRESS_DATA).is_empty());
    }

    #[test]
    fn force_level_moves_the_experience_and_the_level() {
        let mut e = Engine::new();
        let (player, data) = setup(&mut e, 4, 2500);
        e.register(DATA_TEST_008D55E0, |_, _| 1u32.into_ret());
        e.mem.set_u8(data, 1);
        e.call_log = Some(vec![]);
        e.call(0x008d_5300, &args![0x100u32, 4u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x0300_2000).is_empty());

        e.call_log = Some(vec![]);
        e.call(0x008d_5300, &args![0x100u32, 0x0001_0007u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, PLAYER_PROGRESSION_00464E10), vec![vec![player]]);
        assert_eq!(calls(&log, WORD_AT)[0], vec![0x0600_0000]);
        let required = 0x0001_0006u32.wrapping_mul(1000);
        assert_eq!(
            calls(&log, 0x0300_2000),
            vec![vec![player, 0x18, required.wrapping_sub(2500), 0]]
        );
        assert_eq!(e.mem.u8(data + 8), 0);
        assert_eq!(e.mem.u8(data), 0);
        assert_eq!(e.mem.u32(data + 4), 0x0001_0008u32.wrapping_mul(1000));
        assert_eq!(
            calls(&log, BASE_DATA_SET_LEVEL_0047DFE0),
            vec![vec![0x0500_0030, 7]]
        );
        assert_eq!(calls(&log, DATA_TEST_008D55E0).len(), 1);
        assert_eq!(calls(&log, UPDATE_PLAYER_NAME).len(), 1);
        assert_eq!(calls(&log, UPDATE_PLAYER_TITLE).len(), 1);
    }
}
