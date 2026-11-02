//! The once-only tutorial messages: the interface manager's tutorial
//! manager (`InterfaceManager::TutorialManager`, Xbox PDB; at the PC
//! interface manager's `+0x4d4`), read from FalloutNV.exe 1.4.0.525.
//!
//! * Each message has an id (`Interface::TutorialMessageID`, 0–40; the
//!   [`id`] constants) and is the message record `0x168 + id` of
//!   FalloutNV.esm (`HelpHacking`, `HelpCaravanBetting`…).
//! * The manager keeps a word per id: bit 0 "asked for", bit 1 "shown",
//!   bits 2–7 the menu it waits for (its class − 1001; 0 any), bits 8–31 a
//!   delay in milliseconds; then the id it's about to show (41: none) and
//!   the tick it's due.
//! * A menu asks for its message as it opens ([`Tutorials::show_message`],
//!   `00718630`): only a message whose record has "Auto Display" set and
//!   that hasn't been shown is asked for, so the vanilla messages without
//!   it (barter, containers, dialogue, levelling, the Pip-Boy's stats…)
//!   never come up. The menu waits (its own flag set from the answer) until
//!   the message has been shown ([`Tutorials::is_shown`], `00718840`).
//! * Every frame the interface manager's update runs
//!   [`Tutorials::update`] (`007182e0`): once the chosen message is due,
//!   its menu on top and the top menu shown, the tutorial menu opens with
//!   it (`TutorialMenu::Create`, `007e8890`) and it's marked shown
//!   (`007185e0`); before that a message asked for is chosen.
//! * Caravan and the crafting menu open the tutorial menu themselves when
//!   their message hasn't been shown, and mark it.
//! * The saved game keeps only which have been shown (`007187a0`,
//!   `00718890`: bit 1 of each, eight to a byte).
//!
//! `bHelpEnabled:Interface` (`011db094`) exists in the exe but nothing
//! reads it.

use esm::FormId;

/// How many tutorial messages there are (`TUT_COUNT`).
pub const COUNT: usize = 41;

/// The tutorial ids (`Interface::TutorialMessageID`, Xbox PDB).
pub mod id {
    pub const PIPBOY_STATUS: u8 = 0;
    pub const PIPBOY_SPECIAL: u8 = 1;
    pub const PIPBOY_SKILLS: u8 = 2;
    pub const PIPBOY_PERKS: u8 = 3;
    pub const PIPBOY_GENERAL: u8 = 4;
    pub const PIPBOY_ITEMS: u8 = 5;
    pub const PIPBOY_REPAIR: u8 = 6;
    pub const PIPBOY_LOCALMAP: u8 = 7;
    pub const PIPBOY_WORLDMAP: u8 = 8;
    pub const PIPBOY_QUESTS: u8 = 9;
    pub const PIPBOY_NOTES: u8 = 10;
    pub const PIPBOY_RADIO: u8 = 11;
    pub const CHARGEN_ATTRIBUTES: u8 = 12;
    pub const CHARGEN_TRAITS: u8 = 13;
    pub const CHARGEN_SKILLS: u8 = 14;
    pub const CHARGEN_RACE: u8 = 15;
    pub const LEVELING: u8 = 16;
    pub const DIALOGUE: u8 = 17;
    pub const SURGERY: u8 = 18;
    pub const HACKING: u8 = 19;
    pub const LOCKPICKING_PC: u8 = 20;
    pub const VATS_PC: u8 = 21;
    pub const CONTAINER: u8 = 22;
    pub const BARTER: u8 = 23;
    pub const TERMINAL: u8 = 24;
    pub const PIPBOY_STATS: u8 = 25;
    pub const PIPBOY_DATA: u8 = 26;
    pub const VATS_XBOX: u8 = 27;
    pub const LOCKPICKING_XBOX: u8 = 28;
    pub const PIPBOY_ITEMMOD: u8 = 29;
    pub const CARAVAN_BET: u8 = 30;
    pub const CARAVAN_DECK: u8 = 31;
    pub const CARAVAN_TRACK: u8 = 32;
    pub const CARAVAN_GAME: u8 = 33;
    pub const WEAPONS: u8 = 34;
    pub const APPAREL: u8 = 35;
    pub const AMMO: u8 = 36;
    pub const AMMO_XBOX: u8 = 37;
    pub const RECIPE: u8 = 38;
    pub const REPUTATION: u8 = 39;
    pub const HARDCORE_NEEDS: u8 = 40;
}

/// The menus the game's callers ask for their messages over, with the
/// delay they give (milliseconds): `HackingMenu::Create` (`00765b80`:
/// hacking, 512), the lockpicking menu's update (`0078eb50`: lockpicking,
/// none), `ComputersMenu` (`00757b70`: terminal, 512), `BarterMenu`
/// (`0072d250`: barter, 512), `ContainerMenu` (`0075b310`: container,
/// 512), `DialogMenu` (`00762950`: dialogue), `LevelUpMenu` (`00784c80`:
/// levelling, 512), `VATSMenu::Create` (`007e9200`: V.A.T.S., 512),
/// `InventoryMenu::Create` (`0077fc10`: weapons, 512) and its tab
/// buttons (`00780140`: apparel 512, ammo 500), the reputation title's
/// box (`006155f0`: reputation, any menu, 500). (The vanilla barter,
/// container, terminal, dialogue and levelling messages aren't "Auto
/// Display", so never come up.)
pub mod menu {
    /// The message box (`MessageMenu`): what "any menu" waits for when no
    /// message is about to be shown (`007182e0`).
    pub const MESSAGE: i32 = 1001;
    pub const INVENTORY: i32 = 1002;
    pub const VATS: i32 = 1056;
    pub const LOCKPICK: i32 = 1014;
    pub const HACKING: i32 = 1055;
    pub const COMPUTERS: i32 = 1057;
    pub const BARTER: i32 = 1053;
    pub const CONTAINER: i32 = 1008;
    pub const DIALOG: i32 = 1009;
    pub const LEVEL_UP: i32 = 1027;
    /// The delay most menus give.
    pub const DELAY: u32 = 512;
}

/// The message record an id shows (`00718630`, `007182e0`: the form
/// `0x168 + id`, looked up by form ID in the master file).
pub fn message_form(id: u8) -> FormId {
    FormId(0x168 + u32::from(id))
}

/// The help manual the start menu's Help pages through: the form list
/// `HelpManual` (0x163; `HelpManualXBox` 0x165 with a pad: `007e8890`,
/// `007d0770`).
pub const HELP_MANUAL: FormId = FormId(0x163);
pub const HELP_MANUAL_XBOX: FormId = FormId(0x165);

/// The message record's "Auto Display" flag (`BGSMessage::iFlags` bit 1,
/// the `DNAM` flags' 2): only such messages are asked for.
pub const AUTO_DISPLAY: u32 = 2;

/// The lowest and highest menu class a message can wait for (`00718630`:
/// above 1000 and below 1085; stored as the class − 1001 in six bits).
const FIRST_MENU: i32 = 1001;
const LAST_MENU: i32 = 1084;

/// "None about to be shown" (`+0xa4`).
const NONE: usize = COUNT;

/// The bits of a message's word.
const ASKED: u32 = 1;
const SHOWN: u32 = 2;

/// The tutorial manager.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tutorials {
    /// A word per id (`+0x00`..`+0xa0`).
    words: [u32; COUNT],
    /// The id about to be shown (`+0xa4`; [`COUNT`] none).
    current: usize,
    /// When it's due, in the clock's milliseconds (`+0xa8`; 0 not set).
    due: u32,
}

impl Default for Tutorials {
    fn default() -> Tutorials {
        Tutorials {
            words: [0; COUNT],
            current: NONE,
            due: 0,
        }
    }
}

/// What the update needs to know of the menus on screen.
pub trait Screen {
    /// Whether a menu is the one on top (`00702450`).
    fn is_menu_open(&self, class: i32) -> bool;
    /// Whether there is a top menu and it has finished opening (`007024e0`:
    /// its fade state, `Menu` +0x24, is 1).
    fn top_menu_shown(&self) -> bool;
}

impl Tutorials {
    /// The word for an id; past the end, the manager's next field (the id
    /// about to be shown), as the game's own reads go (`007182e0` reads
    /// `+0xa4` as id 41's word when none is about to be shown).
    fn word(&self, i: usize) -> u32 {
        match i {
            i if i < COUNT => self.words[i],
            NONE => self.current as u32,
            _ => 0,
        }
    }

    /// The menu a word waits for (bits 2–7), 0 for any.
    fn menu_bits(word: u32) -> i32 {
        ((word >> 2) & 0x3f) as i32
    }

    /// Asks for a message (`ShowMessage`, `00718630`): `menu` the class of
    /// the menu it's for (0 any), `delay` milliseconds after it's chosen,
    /// `auto_display` whether its record exists and has [`AUTO_DISPLAY`]
    /// set. True when it's now waiting to be shown (the caller then waits
    /// for [`Tutorials::is_shown`]).
    pub fn show_message(&mut self, id: u8, menu: i32, delay: u32, auto_display: bool) -> bool {
        let i = usize::from(id);
        if i >= COUNT {
            return false;
        }
        let w = &mut self.words[i];
        *w = (*w & 0xff) | ((delay & 0x00ff_ffff) << 8);
        let mut asked = false;
        if (FIRST_MENU..=LAST_MENU).contains(&menu) || menu == 0 {
            let bits = if menu == 0 {
                0
            } else {
                ((menu - FIRST_MENU) as u32 & 0x3f) << 2
            };
            *w = (*w & 0xffff_ff03) | bits;
            asked = auto_display && *w & SHOWN == 0;
        }
        *w = (*w & !ASKED) | u32::from(asked);
        asked
    }

    /// Whether a message has been shown (`00718840`).
    pub fn is_shown(&self, id: u8) -> bool {
        self.words
            .get(usize::from(id))
            .is_some_and(|w| w & SHOWN != 0)
    }

    /// Marks a message shown (`007185e0`): no longer asked for. False for
    /// an id out of range.
    pub fn mark_shown(&mut self, id: u8) -> bool {
        match self.words.get_mut(usize::from(id)) {
            Some(w) => {
                *w = (*w & !ASKED) | SHOWN;
                true
            }
            None => false,
        }
    }

    /// The manager's update (`007182e0`), every frame: `now` the clock's
    /// milliseconds (`GetTickCount`); `in_game` false outside the game (the
    /// main menu, loading: the choice is dropped); `held` true while the
    /// pause menu or the name entry is up (nothing happens). The id to show
    /// now, if any: the caller opens the tutorial menu with its message and,
    /// if it opened, marks it with [`Tutorials::mark_shown`].
    pub fn update(
        &mut self,
        now: u32,
        in_game: bool,
        held: bool,
        screen: &dyn Screen,
    ) -> Option<u8> {
        if held {
            return None;
        }
        // The menu a word waits for is open (or it waits for none).
        let open = |bits: i32| bits == 0 || screen.is_menu_open(bits + FIRST_MENU);
        let mut again = true;
        while again {
            again = false;
            if !in_game {
                self.due = 0;
                self.current = NONE;
                continue;
            }
            let ready = self.due != 0
                && now >= self.due
                && self.current < COUNT
                && open(Self::menu_bits(self.word(self.current)))
                && screen.top_menu_shown();
            if ready {
                let id = self.current as u8;
                self.due = 0;
                self.current = NONE;
                return Some(id);
            }
            for i in 0..COUNT {
                let w = self.words[i];
                // The current word's menu decides whether the candidate's
                // menu is looked at (the game's own test).
                if w & ASKED != 0
                    && w & SHOWN == 0
                    && self.current != i
                    && (Self::menu_bits(self.word(self.current)) == 0
                        || screen.is_menu_open(Self::menu_bits(w) + FIRST_MENU))
                    && screen.top_menu_shown()
                {
                    // The one chosen before is dropped (if its menu is up).
                    if self.current < COUNT && open(Self::menu_bits(self.words[self.current])) {
                        self.words[self.current] &= !ASKED;
                    }
                    self.current = i;
                    let delay = w >> 8;
                    self.due = now.wrapping_add(delay);
                    again = delay == 0;
                }
            }
        }
        None
    }

    /// The first id whose word waits for a menu (`007d09c0`, for the
    /// start menu's Help): [`COUNT`] for none. A word that waits for no
    /// menu (bits 0) counts as waiting for the message box (1001), as the
    /// game's own sum makes it.
    pub fn menu_message(&self, class: i32) -> u8 {
        (0..COUNT)
            .find(|&i| Self::menu_bits(self.words[i]) + FIRST_MENU == class)
            .unwrap_or(COUNT) as u8
    }

    /// The saved bytes (`007187a0`): bit 1 of each id, eight to a byte, id
    /// 8n + b in byte n's bit b; six bytes.
    pub fn saved_bytes(&self) -> [u8; COUNT.div_ceil(8)] {
        let mut out = [0u8; COUNT.div_ceil(8)];
        for (i, w) in self.words.iter().enumerate() {
            if w & SHOWN != 0 {
                out[i / 8] |= 1 << (i % 8);
            }
        }
        out
    }

    /// The saved bytes back (`00718890`): only the "shown" bits change.
    pub fn load_bytes(&mut self, bytes: &[u8]) {
        for (i, w) in self.words.iter_mut().enumerate() {
            let on = bytes.get(i / 8).is_some_and(|b| b & (1 << (i % 8)) != 0);
            *w = (*w & !SHOWN) | if on { SHOWN } else { 0 };
        }
    }
}

/// Whether a message record has "Auto Display" set (`BGSMessage` +0x38,
/// the `DNAM` flags).
pub fn auto_display(order: &esm::LoadOrder, message: FormId) -> bool {
    order
        .get(message)
        .and_then(|r| r.record().ok())
        .and_then(|r| {
            let d = &r.get(esm::FourCC::new(b"DNAM"))?.data;
            Some(u32::from_le_bytes(d.get(..4)?.try_into().ok()?))
        })
        .is_some_and(|flags| flags & AUTO_DISPLAY != 0)
}

/// Asks for a message as a menu does, reading its record's flag
/// ([`Tutorials::show_message`] with [`auto_display`] of
/// [`message_form`]).
pub fn ask(
    order: &esm::LoadOrder,
    tutorials: &mut Tutorials,
    id: u8,
    menu: i32,
    delay: u32,
) -> bool {
    let auto = auto_display(order, message_form(id));
    tutorials.show_message(id, menu, delay, auto)
}

pub(crate) fn save_lines(state: &crate::scripting::GameState, line: &mut dyn FnMut(String)) {
    let bytes = state.tutorials.saved_bytes();
    if bytes.iter().any(|&b| b != 0) {
        let hex: String = bytes.iter().map(|b| format!("{b:02X}")).collect();
        line(format!("tutorials {hex}"));
    }
}

pub(crate) fn load_line(
    state: &mut crate::scripting::GameState,
    raw: &str,
) -> Option<Result<(), String>> {
    let mut parts = raw.split_whitespace();
    if parts.next() != Some("tutorials") {
        return None;
    }
    let hex = parts.next().unwrap_or("");
    let bytes: Option<Vec<u8>> = (0..hex.len())
        .step_by(2)
        .map(|i| {
            hex.get(i..i + 2)
                .and_then(|b| u8::from_str_radix(b, 16).ok())
        })
        .collect();
    match bytes {
        Some(b) if !b.is_empty() => {
            state.tutorials.load_bytes(&b);
            Some(Ok(()))
        }
        _ => Some(Err(format!("can't read '{raw}'"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A screen with one menu on top (or none), shown.
    struct Top(Option<i32>);
    impl Screen for Top {
        fn is_menu_open(&self, class: i32) -> bool {
            self.0 == Some(class)
        }
        fn top_menu_shown(&self) -> bool {
            self.0.is_some()
        }
    }

    const HACKING_MENU: i32 = 1055;
    const MESSAGE_MENU: i32 = 1001;

    #[test]
    fn asking_needs_auto_display_and_not_shown() {
        let mut t = Tutorials::default();
        assert!(!t.show_message(id::BARTER, 1053, 512, false));
        assert!(t.show_message(id::HACKING, HACKING_MENU, 512, true));
        t.mark_shown(id::HACKING);
        assert!(t.is_shown(id::HACKING));
        assert!(!t.show_message(id::HACKING, HACKING_MENU, 512, true));
        // Out of range.
        assert!(!t.show_message(41, 0, 0, true));
        assert!(!t.is_shown(41));
        // A menu out of 1001..=1084 isn't stored and isn't asked for.
        assert!(!t.show_message(id::WEAPONS, 1085, 0, true));
    }

    #[test]
    fn a_message_waits_for_its_menu_and_its_delay() {
        let mut t = Tutorials::default();
        assert!(t.show_message(id::HACKING, HACKING_MENU, 512, true));
        // Not on top: nothing is chosen.
        assert_eq!(t.update(1000, true, false, &Top(Some(1009))), None);
        assert_eq!(t.current, NONE);
        // On top: chosen, due in 512 ms.
        assert_eq!(t.update(1000, true, false, &Top(Some(HACKING_MENU))), None);
        assert_eq!((t.current, t.due), (usize::from(id::HACKING), 1512));
        assert_eq!(t.update(1511, true, false, &Top(Some(HACKING_MENU))), None);
        // Held (the pause menu): nothing.
        assert_eq!(t.update(1600, true, true, &Top(Some(HACKING_MENU))), None);
        assert_eq!(
            t.update(1512, true, false, &Top(Some(HACKING_MENU))),
            Some(id::HACKING)
        );
        assert_eq!(t.current, NONE);
        // The caller marks it; it isn't chosen again.
        t.mark_shown(id::HACKING);
        assert_eq!(t.update(2000, true, false, &Top(Some(HACKING_MENU))), None);
    }

    #[test]
    fn no_delay_shows_on_the_same_update() {
        let mut t = Tutorials::default();
        // The lockpick menu asks with no delay.
        assert!(t.show_message(id::LOCKPICKING_PC, 1014, 0, true));
        assert_eq!(
            t.update(5000, true, false, &Top(Some(1014))),
            Some(id::LOCKPICKING_PC)
        );
    }

    #[test]
    fn a_message_for_any_menu_waits_for_a_message_box() {
        // With none chosen the manager's own next field is read as id 41's
        // word (41: its menu bits are 10), so a message for any menu (0)
        // needs the message box (class 1001 + 0) on top: the reputation
        // message comes up over the reputation change's box.
        let mut t = Tutorials::default();
        assert!(t.show_message(id::REPUTATION, 0, 500, true));
        assert_eq!(t.update(100, true, false, &Top(None)), None);
        assert_eq!(t.update(100, true, false, &Top(Some(1004))), None);
        assert_eq!(t.current, NONE);
        assert_eq!(t.update(100, true, false, &Top(Some(MESSAGE_MENU))), None);
        assert_eq!(t.current, usize::from(id::REPUTATION));
        // Chosen, it shows over any menu once due.
        assert_eq!(
            t.update(600, true, false, &Top(Some(1002))),
            Some(id::REPUTATION)
        );
    }

    #[test]
    fn leaving_the_game_drops_the_choice() {
        let mut t = Tutorials::default();
        t.show_message(id::HACKING, HACKING_MENU, 512, true);
        t.update(0, true, false, &Top(Some(HACKING_MENU)));
        assert_eq!(t.current, usize::from(id::HACKING));
        assert_eq!(t.update(1000, false, false, &Top(Some(HACKING_MENU))), None);
        assert_eq!((t.current, t.due), (NONE, 0));
        // Still asked for: chosen again once back.
        assert_eq!(t.update(1000, true, false, &Top(Some(HACKING_MENU))), None);
        assert_eq!(t.current, usize::from(id::HACKING));
    }

    #[test]
    fn a_later_choice_drops_the_earlier_one() {
        // Two asked for the same menu: the scan ends on the last, and the
        // earlier choice is no longer asked for.
        let mut t = Tutorials::default();
        t.show_message(id::WEAPONS, 1002, 512, true);
        t.show_message(id::AMMO, 1002, 500, true);
        assert_eq!(t.update(0, true, false, &Top(Some(1002))), None);
        assert_eq!(t.current, usize::from(id::AMMO));
        assert_eq!(t.words[usize::from(id::WEAPONS)] & ASKED, 0);
        assert_eq!(t.due, 500);
    }

    #[test]
    fn only_shown_bits_are_saved() {
        let mut t = Tutorials::default();
        t.mark_shown(id::HACKING);
        t.mark_shown(id::CARAVAN_BET);
        t.mark_shown(id::HARDCORE_NEEDS);
        t.show_message(id::WEAPONS, 1002, 512, true);
        let bytes = t.saved_bytes();
        assert_eq!(bytes, [0, 0, 0b1000, 0b0100_0000, 0, 1]);
        let mut back = Tutorials::default();
        back.load_bytes(&bytes);
        for i in 0..COUNT as u8 {
            assert_eq!(back.is_shown(i), t.is_shown(i), "id {i}");
        }
        assert!(!back.show_message(id::HACKING, HACKING_MENU, 0, true));
    }

    #[test]
    fn shown_messages_go_into_the_saved_game() {
        let mut state = crate::scripting::GameState::default();
        assert!(!crate::save::save(&state, None).contains("tutorials"));
        state.tutorials.mark_shown(id::CARAVAN_DECK);
        state.tutorials.mark_shown(id::LOCKPICKING_PC);
        let text = crate::save::save(&state, None);
        assert!(text.contains("tutorials 000010800000"), "{text}");
        let (back, _) = crate::save::load(&text).unwrap();
        assert!(back.tutorials.is_shown(id::CARAVAN_DECK));
        assert!(back.tutorials.is_shown(id::LOCKPICKING_PC));
        assert!(!back.tutorials.is_shown(id::HACKING));
        assert!(crate::save::load("nv-rs save 1\ntutorials zz\n").is_err());
    }

    /// A master file with two of the help messages: hacking with "Auto
    /// Display" (`DNAM` 3, as vanilla), barter without (`DNAM` 1).
    fn messages() -> esm::LoadOrder {
        use testdata::{group, record, sub, zstr};
        let mut header = 1.34f32.to_le_bytes().to_vec();
        header.extend([0; 8]);
        let mut bytes = record(b"TES4", 0, &sub(b"HEDR", &header));
        let mesg = |id: u8, edid: &str, flags: u32| {
            let mut d = sub(b"EDID", &zstr(edid));
            d.extend(sub(b"DESC", &zstr("Text")));
            d.extend(sub(b"FULL", &zstr("Title")));
            d.extend(sub(b"DNAM", &flags.to_le_bytes()));
            record(b"MESG", message_form(id).0, &d)
        };
        let mut g = mesg(id::HACKING, "HelpHacking", 3);
        g.extend(mesg(id::BARTER, "HelpBarter", 1));
        bytes.extend(group(*b"MESG", 0, &g));
        let plugin = esm::Plugin::from_bytes(bytes).unwrap();
        esm::LoadOrder::single("FalloutNV.esm", None, plugin).unwrap()
    }

    #[test]
    fn the_record_decides_whether_it_is_asked_for() {
        let order = messages();
        let mut t = Tutorials::default();
        assert!(ask(&order, &mut t, id::HACKING, HACKING_MENU, 512));
        assert!(!ask(&order, &mut t, id::BARTER, 1053, 512));
        // No record at all.
        assert!(!ask(&order, &mut t, id::WEAPONS, 1002, 512));
    }

    #[test]
    fn message_forms() {
        assert_eq!(message_form(id::CARAVAN_BET), FormId(0x186));
        assert_eq!(message_form(id::HACKING), FormId(0x17B));
        assert_eq!(message_form(id::HARDCORE_NEEDS), FormId(0x190));
    }

    /// The start menu's Help looks for the first message a menu asked
    /// for (`007d09c0`), whether or not it's been shown.
    #[test]
    fn the_message_a_menu_asked_for() {
        let mut t = Tutorials::default();
        assert_eq!(t.menu_message(menu::VATS), COUNT as u8);
        t.show_message(id::VATS_PC, menu::VATS, 512, true);
        t.mark_shown(id::VATS_PC);
        assert_eq!(t.menu_message(menu::VATS), id::VATS_PC);
        t.show_message(id::AMMO, menu::INVENTORY, 500, true);
        t.show_message(id::WEAPONS, menu::INVENTORY, 512, true);
        assert_eq!(t.menu_message(menu::INVENTORY), id::WEAPONS);
        // No menu: the message box's.
        assert_eq!(t.menu_message(menu::MESSAGE), 0);
    }
}
