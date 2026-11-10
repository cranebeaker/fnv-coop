//! The story kit's stand-ins for conversations, saves and rests (feature `testkit`), laid out as
//! the game lays them out where co-op reads them. Each does what co-op relies on the real one to
//! do, and no more:
//!
//! - `TESTopicInfo::RunResult` counts the line as said (index 0) and runs the info's result
//!   script through `ScriptRunner::Run`. The script has no owner quest and is no quest script,
//!   as an info's result script in the game: on a guest only the apply scope keeps it from
//!   running. What the script does (a quest stage) happens only when the runner really ran it.
//! - `DialogMenu::Create` opens the one dialogue menu on its speaker; `DialogMenu::Close` closes
//!   it at once.
//! - `SleepWaitMenu::DoClick` starts the rest (button 4: the player's wait is set) or cancels it
//!   (button 5); the player's hour step moves the hour global on by one.
//! - The HUD message keeps its text, cut as the game cuts it.

use engine_core::prelude::*;

use crate::testkit::Story;
use crate::{addr, forms};

/// The kit's own memory.
const KIT: u32 = 0x0300_0000;
const KIT_LEN: u32 = 0x0020_0000;
/// Counters and settings: infos made, messages shown, the hour global's form, rest clicks,
/// cancel clicks, hour steps, script passes.
const INFOS_MADE: u32 = KIT;
const MESSAGES_SHOWN: u32 = KIT + 4;
const HOUR_GLOBAL: u32 = KIT + 8;
const REST_CLICKS: u32 = KIT + 0xc;
const CANCEL_CLICKS: u32 = KIT + 0x10;
const HOUR_STEPS: u32 = KIT + 0x14;
const SCRIPT_PASSES: u32 = KIT + 0x18;
/// The dialogue menu, the sleep and wait menu, its hours tile, and a save's name.
const DIALOG_MENU: u32 = KIT + 0x100;
const SLEEP_MENU: u32 = KIT + 0x200;
const HOURS_TILE: u32 = KIT + 0x300;
const SAVE_NAME: u32 = KIT + 0x400;
/// The messages shown: 0x110 bytes each.
const MESSAGES: u32 = KIT + 0x1000;
const MESSAGE_SLOT: u32 = 0x110;
const MESSAGES_MAX: u32 = 64;
/// The infos: 0x400 bytes each. The form at +0; how often it was said at +0x60; its quest (a
/// form) at +0x64; the quest and stage its result at index 0 sets at +0x70, at index 1 at +0x78;
/// its "topic" at +0x80; its two result scripts at +0x100 and +0x180; the prompt's characters
/// at +0x200 and the response text at +0x280.
const INFOS: u32 = KIT + 0x1_0000;
const INFO_SLOT: u32 = 0x400;
const INFO_SAID: u32 = 0x60;
const INFO_QUEST: u32 = 0x64;
const INFO_RESULTS: u32 = 0x70;
const INFO_TOPIC: u32 = 0x80;
const INFO_SCRIPTS: u32 = 0x100;
const INFO_PROMPT_TEXT: u32 = 0x200;
const INFO_TEXT: u32 = 0x280;
/// `PlayerCharacter`: the hours left to wait at +0x654 and the bed flag at +0x658
/// (`tesscriptfunctions_p2.rs`, `fn_005c1a00`).
const PLAYER_WAIT_HOURS: u32 = 0x654;
const PLAYER_WAIT_IN_BED: u32 = 0x658;

fn eax(v: u32) -> Ret {
    Ret {
        eax: v,
        ..Ret::default()
    }
}

fn bump(e: &mut Engine, at: u32) {
    let n = e.mem.u32(at);
    e.mem.set_u32(at, n + 1);
}

fn run_result(e: &mut Engine, a: &[u32]) -> Ret {
    let (info, index, reference) = (a[0], a[1].min(1), a[2]);
    if reference == 0 {
        return Ret::default();
    }
    if index == 0 {
        bump(e, info + INFO_SAID);
    }
    let result = info + INFO_RESULTS + 8 * index;
    let quest = e.mem.u32(result);
    if quest != 0 {
        let before = e.mem.u32(Story::RAN_AT);
        let script = info + INFO_SCRIPTS + 0x80 * index;
        e.call(
            addr::SCRIPT_RUNNER_RUN,
            &args![0u32, script, reference, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        if e.mem.u32(Story::RAN_AT) != before {
            let stage = e.mem.u32(result + 4);
            e.call(addr::QUEST_SET_STAGE_DONE, &args![quest, stage]);
        }
    }
    Ret::default()
}

fn menu_create(e: &mut Engine, a: &[u32]) -> Ret {
    if a[0] == 0 {
        return eax(0);
    }
    e.mem
        .set_u32(DIALOG_MENU + addr::DIALOG_MENU_SPEAKER_OFFSET, a[0]);
    e.mem.set_u32(addr::DIALOG_MENU_OPEN, DIALOG_MENU);
    eax(1)
}

fn menu_close(e: &mut Engine, _: &[u32]) -> Ret {
    e.mem.set_u32(addr::DIALOG_MENU_OPEN, 0);
    Ret::default()
}

fn owner_topic(_: &mut Engine, a: &[u32]) -> Ret {
    eax(a[0] + INFO_TOPIC)
}

fn topic_info_quest(e: &mut Engine, a: &[u32]) -> Ret {
    eax(e.mem.u32(a[1] + INFO_QUEST))
}

fn display_text(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem.set_u32(a[1], a[0] + INFO_TEXT);
    Ret::default()
}

fn string_clear(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem.set_u32(a[0], 0);
    e.mem.set_u32(a[0] + 4, 0);
    Ret::default()
}

fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
    Ret::default()
}

fn hud_message(e: &mut Engine, a: &[u32]) -> Ret {
    let n = e.mem.u32(MESSAGES_SHOWN);
    if a[0] != 0 && n < MESSAGES_MAX {
        let mut text = e.mem.cstr(a[0]);
        text.truncate(addr::HUD_MESSAGE_MAX);
        e.mem.set_cstr(MESSAGES + MESSAGE_SLOT * n, &text);
        e.mem.set_u32(MESSAGES_SHOWN, n + 1);
    }
    Ret::default()
}

fn player(e: &mut Engine) -> u32 {
    e.mem.u32(addr::PLAYER_SINGLETON)
}

fn do_click(e: &mut Engine, a: &[u32]) -> Ret {
    let menu = a[0];
    let player = player(e);
    match a[1] {
        4 => {
            bump(e, REST_CLICKS);
            let tile = e.mem.u32(menu + addr::SLEEP_WAIT_HOURS_TILE_OFFSET);
            let id = e.mem.u32(addr::SLEEP_WAIT_VALUE_TRAIT);
            let value = e.call(addr::TILE_GET_FLOAT, &args![tile, id]).f64();
            let bed = u32::from(e.mem.u8(menu + addr::SLEEP_WAIT_IN_BED_OFFSET));
            e.call(addr::PLAYER_SET_WAIT, &args![player, value as u32 + 1, bed]);
        }
        5 => {
            bump(e, CANCEL_CLICKS);
            e.call(addr::PLAYER_SET_WAIT, &args![player, 0u32, 0u32]);
        }
        _ => {}
    }
    Ret::default()
}

fn tile_get_float(e: &mut Engine, a: &[u32]) -> Ret {
    Ret {
        st0: f64::from(e.mem.f32(a[0] + 0x10)),
        ..Ret::default()
    }
}

fn set_wait(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem.set_u32(a[0] + PLAYER_WAIT_HOURS, a[1]);
    e.mem.set_u8(a[0] + PLAYER_WAIT_IN_BED, a[2] as u8);
    Ret::default()
}

fn wait_hour(e: &mut Engine, a: &[u32]) -> Ret {
    bump(e, HOUR_STEPS);
    let global = e.mem.u32(HOUR_GLOBAL);
    if global != 0 {
        let mut hour = e.mem.f32(global + addr::GLOBAL_VALUE_OFFSET) + 1.0;
        if hour >= 24.0 {
            hour -= 24.0;
        }
        e.mem.set_f32(global + addr::GLOBAL_VALUE_OFFSET, hour);
    }
    let left = e.mem.u32(a[0] + PLAYER_WAIT_HOURS);
    e.mem
        .set_u32(a[0] + PLAYER_WAIT_HOURS, left.saturating_sub(1));
    Ret::default()
}

fn run_scripts(e: &mut Engine, _: &[u32]) -> Ret {
    bump(e, SCRIPT_PASSES);
    Ret::default()
}

/// Maps the kit's memory and registers its stand-ins; before co-op is installed.
pub(crate) fn register(e: &mut Engine) {
    e.map(KIT, KIT_LEN);
    e.map(addr::DIALOG_MENU_OPEN & !0xfff, 0x1000);
    e.map(addr::SLEEP_WAIT_VALUE_TRAIT & !0xfff, 0x1000);
    e.mem.set_u32(addr::SLEEP_WAIT_VALUE_TRAIT, 0xfc4);
    e.mem.set_u32(addr::TES_SINGLETON, KIT + 0x500);
    e.mem
        .set_u32(SLEEP_MENU + addr::SLEEP_WAIT_HOURS_TILE_OFFSET, HOURS_TILE);
    e.register(addr::INFO_RUN_RESULT, run_result);
    e.register(addr::DIALOG_MENU_CREATE, menu_create);
    e.register(addr::DIALOG_MENU_CLOSE, menu_close);
    e.register(addr::TOPIC_GET_OWNER_TOPIC, owner_topic);
    e.register(addr::TOPIC_INFO_QUEST, topic_info_quest);
    e.register(addr::INFO_GET_DISPLAY_TEXT, display_text);
    e.register(addr::STRING_CONSTRUCT, string_clear);
    e.register(addr::STRING_DESTROY, string_clear);
    e.register(addr::OPERATOR_DELETE, nothing);
    e.register(addr::SHOW_HUD_MESSAGE, hud_message);
    e.register(addr::SLEEP_WAIT_DO_CLICK, do_click);
    e.register(addr::TILE_GET_FLOAT, tile_get_float);
    e.register(addr::PLAYER_SET_WAIT, set_wait);
    e.register(addr::PLAYER_WAIT_HOUR, wait_hour);
    e.register(addr::TES_RUN_SCRIPTS, run_scripts);
}

impl Story {
    fn info_at(&mut self, id: u32) -> u32 {
        let f = forms::form(&mut self.e, id).expect("an info of the story");
        assert_eq!(
            self.e.mem.u8(f + addr::FORM_TYPE_OFFSET),
            addr::FORM_TYPE_INFO
        );
        f
    }

    /// Adds a line of dialogue: info `id` of quest `quest` (0 for none), with its response text
    /// and its prompt (what the player chose; empty for none).
    pub fn add_info(&mut self, id: u32, quest: u32, text: &str, prompt: &str) {
        let n = self.e.mem.u32(INFOS_MADE);
        let info = INFOS + INFO_SLOT * n;
        assert!(
            info + INFO_SLOT <= KIT + KIT_LEN,
            "a test story's infos fit"
        );
        assert!(text.len() < 0x100 && prompt.len() < 0x80);
        self.e.mem.set_u32(INFOS_MADE, n + 1);
        self.e
            .mem
            .set_u8(info + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_INFO);
        self.e.mem.set_u32(info + addr::FORM_ID_OFFSET, id);
        if quest != 0 {
            let q = forms::quest(&mut self.e, quest).expect("a quest of the story");
            self.e.mem.set_u32(info + INFO_QUEST, q);
        }
        self.e.mem.set_cstr(info + INFO_TEXT, text.as_bytes());
        if !prompt.is_empty() {
            self.e
                .mem
                .set_cstr(info + INFO_PROMPT_TEXT, prompt.as_bytes());
            self.e
                .mem
                .set_u32(info + addr::INFO_PROMPT_OFFSET, info + INFO_PROMPT_TEXT);
        }
        for index in 0..2 {
            self.e.mem.set_u32(
                info + INFO_SCRIPTS + 0x80 * index + addr::FORM_ID_OFFSET,
                (id ^ 0x00e0_0000) + index,
            );
        }
        self.add_form(id, info);
    }

    /// Gives info `id` a result script: when the line is said (`end` false) or ends (`end` true)
    /// it sets `stage` of `quest`.
    pub fn set_info_result(&mut self, id: u32, end: bool, quest: u32, stage: u8) {
        let info = self.info_at(id);
        let q = forms::quest(&mut self.e, quest).expect("a quest of the story");
        let at = info + INFO_RESULTS + 8 * u32::from(end);
        self.e.mem.set_u32(at, q);
        self.e.mem.set_u32(at + 4, u32::from(stage));
    }

    /// Puts a placed reference into `cell`, with its 3D.
    pub fn place(&mut self, reference: u32, cell: u32) {
        let r = forms::form(&mut self.e, reference).expect("a reference of the story");
        let c = forms::form(&mut self.e, cell).expect("a cell of the story");
        self.e.mem.set_u32(r + addr::REF_CELL_OFFSET, c);
        self.e.mem.set_u32(r + addr::REF_LOADED_OFFSET, 1);
    }

    /// The player starts a conversation with `npc`, as activating it does; whether the game
    /// opened its dialogue menu.
    pub fn talk_to(&mut self, npc: u32) -> bool {
        let speaker = forms::form(&mut self.e, npc).expect("a reference of the story");
        self.e
            .call(addr::DIALOG_MENU_CREATE, &args![speaker, 0u32, 0u32])
            .u32()
            != 0
    }

    /// The open conversation's speaker says info `id` (`end` false) or the line ends (`end`
    /// true), as the dialogue menu does.
    pub fn say(&mut self, id: u32, end: bool) {
        let npc = crate::talk::menu_npc(&mut self.e).expect("a conversation is open");
        self.say_as(npc, id, end);
    }

    /// Reference `npc` says info `id`, whatever the dialogue menu shows (an NPC's own line).
    pub fn say_as(&mut self, npc: u32, id: u32, end: bool) {
        let info = self.info_at(id);
        let speaker = forms::form(&mut self.e, npc).expect("a reference of the story");
        self.e
            .call(addr::INFO_RUN_RESULT, &args![info, u32::from(end), speaker]);
    }

    /// The conversation ends by itself (the menu's own goodbye).
    pub fn leave(&mut self) {
        self.e.mem.set_u32(addr::DIALOG_MENU_OPEN, 0);
    }

    /// How often info `id` was said in this game.
    pub fn said(&mut self, id: u32) -> u32 {
        let info = self.info_at(id);
        self.e.mem.u32(info + INFO_SAID)
    }

    /// The HUD messages shown so far, in order.
    pub fn messages(&mut self) -> Vec<String> {
        (0..self.e.mem.u32(MESSAGES_SHOWN))
            .map(|i| {
                self.e
                    .mem
                    .cstr(MESSAGES + MESSAGE_SLOT * i)
                    .into_iter()
                    .map(char::from)
                    .collect()
            })
            .collect()
    }

    fn site(&mut self, at: u32, name: &str) -> Option<u32> {
        assert!(name.len() < 0x200);
        self.e.mem.set_cstr(SAVE_NAME, name.as_bytes());
        self.e.site(at, &[SAVE_NAME])
    }

    /// The game writes a save named `name`: the seam's answer (`None`: nobody took the call).
    pub fn save(&mut self, name: &str) -> Option<u32> {
        self.site(addr::SAVE_GAME_SITE, name)
    }

    /// The game reads the save named `name`: the seam's answer.
    pub fn load(&mut self, name: &str) -> Option<u32> {
        self.site(addr::LOAD_GAME_SITE, name)
    }

    /// Names the global the hour step moves (the game's hour).
    pub fn set_hour_global(&mut self, id: u32) {
        let g = forms::form(&mut self.e, id).expect("a global of the story");
        self.e.mem.set_u32(HOUR_GLOBAL, g);
    }

    /// The player picks `hours` in the sleep and wait menu (in a bed or not) and presses rest.
    pub fn click_rest(&mut self, hours: u32, in_bed: bool) {
        self.e
            .mem
            .set_f32(HOURS_TILE + 0x10, hours.saturating_sub(1) as f32);
        self.e.mem.set_u8(
            SLEEP_MENU + addr::SLEEP_WAIT_IN_BED_OFFSET,
            u8::from(in_bed),
        );
        self.e.call(
            addr::SLEEP_WAIT_DO_CLICK,
            &args![SLEEP_MENU, addr::SLEEP_WAIT_REST_BUTTON, 0u32],
        );
    }

    /// The player presses cancel in the sleep and wait menu.
    pub fn click_cancel(&mut self) {
        self.e.call(
            addr::SLEEP_WAIT_DO_CLICK,
            &args![SLEEP_MENU, addr::SLEEP_WAIT_CANCEL_BUTTON, 0u32],
        );
    }

    /// `(rests the game started, cancels)` in the sleep and wait menu.
    pub fn rest_clicks(&self) -> (u32, u32) {
        (self.e.mem.u32(REST_CLICKS), self.e.mem.u32(CANCEL_CLICKS))
    }

    /// `(hour steps, script passes)` the game made.
    pub fn hours_passed(&self) -> (u32, u32) {
        (self.e.mem.u32(HOUR_STEPS), self.e.mem.u32(SCRIPT_PASSES))
    }

    /// The player's wait: `(hours left, in a bed)`.
    pub fn wait_left(&mut self) -> (u32, bool) {
        let p = player(&mut self.e);
        (
            self.e.mem.u32(p + PLAYER_WAIT_HOURS),
            self.e.mem.u8(p + PLAYER_WAIT_IN_BED) != 0,
        )
    }
}
