//! The dialogue menu (`menus\dialog\dialog_menu.xml`, class `DialogMenu`
//! 1009, vtable `0107257c` in FalloutNV.exe). The conversation itself (who
//! says what, which topics are offered) is the game's dialogue code; the
//! menu shows it. Read from the code:
//!
//! * opening (`00761a20`): the speaker's name in `DM_SpeakerNameLabel` (id
//!   1), the list box on `DM_TopicList` (id 3) with `DM_TopicTemplate`, its
//!   scroll bar at y 40 and its highlight box at x 40, 960 wide;
//!   `_ShowSubtitles` from `[GamePlay] bDialogueSubtitles`; the first line
//!   shown at once unless the camera has to zoom in first;
//! * every frame (`00762950`): the zoom-in (state 0) runs over
//!   `fDialogZoomInSeconds` (1.5); `_DialogVisible` turns on once it is a
//!   tenth of the way (or at the end when the first line waited for it);
//!   closing (state 4) zooms back out over `fDialogZoomOutSeconds` (0.5),
//!   then the menu goes;
//! * a line (`00762ff0`): its text in `DM_SpeakerText` (id 2, a space when
//!   it has none), `_ShowingText` 1 (state 3); after the last response the
//!   topics: `_ShowingText` 0 and the list (`007638b0`, state 2);
//! * the topic list (`007638b0`): each choice an item numbered by its place,
//!   named `topic_<n>`, its `_line_alpha` 128 when dimmed (said before, or
//!   "always darken") else 255, the failed-check flag on a check the player
//!   doesn't pass; the list as high as its first three items (the file keeps
//!   it at least `_MinListHeight`), its scroll bar `_height` the menu's
//!   `_ScrollbarHeight` and back at the top; clicks wait 500 ms after the
//!   list appears;
//! * a click (`007624f0`): on a topic (id -1) chooses it; on the screen
//!   (`DM_ClickRect`, id 0) while a line shows, moves on; Enter does the
//!   same (`007628c0`: special code 9 clicks the chosen topic, or the screen
//!   while a line shows).
//! * a service menu over the conversation (`00763ff0`, called as the
//!   barter menu `0072d250`, the recipe menu `00726ff0`, a companion's
//!   trade `0075bc80` mode 3, the repair menu `007b7570` and the face menu
//!   from dialogue `00705870` are made): unless the conversation is ending
//!   (`+0x2c`), `+0x138` and `+0x13a` set, the line being said cut short
//!   (`0083e4c0`: the speaker stops, `+0x7c` done) and the menu faded out
//!   (`00a1d910`; without trait 6002 it stays open, hidden). Every frame
//!   (`00762950`) state 2 with both flags becomes 5 (`eServiceFadeOut`
//!   (Xbox PDB)); once faded out (`+0x24` 4) the topics are loaded again
//!   (`00762ff0`), `+0x13a` cleared, back to 2. The service menu closing
//!   (`0072d6d0`, `00727430`, `0075b750` mode 3, `007b78e0`, `007ada40`)
//!   calls `007640a0`: `+0x138` cleared and the menu faded back in
//!   (`00a1db20`).

use crate::fade::Fades;
use crate::list::{set_keeping_operators, ListBox};
use crate::menu::{special, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\dialog\\dialog_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1009;
/// The template topics are made from.
pub const TOPIC_TEMPLATE: &str = "DM_TopicTemplate";
/// `fDialogZoomInSeconds` and `fDialogZoomOutSeconds`' defaults (settings
/// `011d3ee0`, `011d243c`).
pub const ZOOM_IN_SECONDS: f32 = 1.5;
pub const ZOOM_OUT_SECONDS: f32 = 0.5;
/// How long clicks wait after the topics appear (`007624f0`).
pub const CLICK_WAIT_MS: f64 = 500.0;
/// How far into the zoom the menu shows (`01072758`).
pub const SHOW_AT: f32 = 0.1;

/// A choice as the menu lists it.
#[derive(Debug, Clone, PartialEq)]
pub struct Topic {
    /// What's shown: the prompt with any check tag before it.
    pub text: String,
    /// Said before or "always darken": drawn at alpha 128.
    pub dim: bool,
    /// A check the player doesn't pass.
    pub failed: bool,
}

/// What the menu is doing (the menu object's `+0x28`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// The camera zooming in.
    ZoomIn,
    /// Moving on: the speaker finishing, then the next line or the topics.
    Next,
    /// The topics are up.
    Topics,
    /// A line is up.
    Line,
    /// Zooming out, then closed.
    Closing,
}

/// What the player did, for the game's dialogue code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    /// Clicked the line away (move on).
    Skip,
    /// Chose the topic at this place in the list.
    Topic(usize),
}

/// The dialogue menu.
#[derive(Debug)]
pub struct DialogMenu {
    pub menu: TileId,
    /// `SetTile` ids 0..3 (`00761900`): `DM_ClickRect`,
    /// `DM_SpeakerNameLabel`, `DM_SpeakerText`, `DM_TopicList`.
    pub tiles: [Option<TileId>; 4],
    pub list: ListBox,
    pub state: State,
    /// Seconds into the zoom (`+0x108`) and how far it has got (`+0x128`).
    zoom_time: f32,
    pub zoom: f32,
    /// The first line waits for the zoom (`+0x139`).
    wait_for_zoom: bool,
    /// When the topics appeared, in seconds (`+0x10c`).
    topics_at: Option<f64>,
    /// What the player did, once.
    pub answer: Option<Answer>,
    /// Closed: the caller takes the menu off the screen.
    pub closed: bool,
    /// A service menu is over the conversation (`+0x138`).
    pub in_service: bool,
    /// The line being said was cut short when the service menu opened
    /// (`00763ff0`), for the conversation to take.
    pub cut_line: bool,
}

impl DialogMenu {
    pub fn new(menu: TileId) -> DialogMenu {
        DialogMenu {
            menu,
            tiles: [None; 4],
            list: ListBox::default(),
            state: State::ZoomIn,
            zoom_time: 0.0,
            zoom: 0.0,
            wait_for_zoom: false,
            topics_at: None,
            answer: None,
            closed: false,
            in_service: false,
            cut_line: false,
        }
    }

    fn custom(ui: &mut Ui, name: &str) -> i32 {
        ui.names.lookup_or_add(name).unwrap_or(0)
    }

    /// Opens the menu (`00761a20`) for a speaker named `name`;
    /// `subtitles` is `[GamePlay] bDialogueSubtitles`; `wait_for_zoom`:
    /// the first line waits for the camera (the zoom setting on).
    pub fn open(&mut self, ui: &mut Ui, name: &str, subtitles: bool, wait_for_zoom: bool) {
        if let Some(list) = self.tiles[3] {
            self.list = ListBox::new(ui, list, TOPIC_TEMPLATE);
            if let Some(bar) = ui.find_below(list, "lb_scrollbar") {
                ui.set_number(bar, t::Y, 40.0);
            }
            if let Some(hl) = ui.find_below(list, "lb_highlight_box") {
                ui.set_number(hl, t::X, 40.0);
                ui.set_number(hl, t::WIDTH, 960.0);
            }
        }
        let show = Self::custom(ui, "_ShowSubtitles");
        ui.set_number(self.menu, show, if subtitles { 1.0 } else { 0.0 });
        if let Some(label) = self.tiles[1] {
            ui.set_text(label, t::STRING, name);
        }
        self.wait_for_zoom = wait_for_zoom;
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
    }

    /// Whether the first line waits for the zoom.
    pub fn waiting_for_zoom(&self) -> bool {
        self.wait_for_zoom
    }

    /// A line to show (`00762ff0` with a response): its text (a space when
    /// it has none) in `DM_SpeakerText`, `_ShowingText` 1.
    pub fn show_line(&mut self, ui: &mut Ui, text: &str) {
        if let Some(tile) = self.tiles[2] {
            let text = if text.is_empty() { " " } else { text };
            ui.set_string(tile, t::STRING, text);
        }
        let showing = Self::custom(ui, "_ShowingText");
        ui.set_number(self.menu, showing, 1.0);
        // The callers set the state from what this returns (`00762ff0`
        // gives 3); while the camera zooms in, the menu stays zooming.
        if self.state != State::ZoomIn {
            self.state = State::Line;
        }
        ui.refresh();
    }

    /// The topics (`007638b0`): `_ShowingText` 0, each choice an item,
    /// the list sized and scrolled to the top.
    pub fn show_topics(&mut self, ui: &mut Ui, topics: &[Topic], now: f64) {
        let showing = Self::custom(ui, "_ShowingText");
        ui.set_number(self.menu, showing, 0.0);
        self.list.clear(ui);
        let line_alpha = Self::custom(ui, "_line_alpha");
        let mut height = 0i32;
        for (i, topic) in topics.iter().enumerate() {
            let Some(tile) = self.list.add(ui, self.menu, i as i32, Some(&topic.text)) else {
                continue;
            };
            ui.tiles[tile].name = format!("topic_{i}");
            ui.set_number(tile, t::LISTINDEX, i as f32);
            ui.set_number(tile, line_alpha, if topic.dim { 128.0 } else { 255.0 });
            if topic.failed {
                ui.tiles[tile].failed = true;
            }
            if i < 3 {
                height += ui.number(tile, t::HEIGHT) as i32;
            }
        }
        if let Some(list) = self.tiles[3] {
            set_keeping_operators(ui, list, t::HEIGHT, height as f32);
        }
        let bar_height = Self::custom(ui, "_ScrollbarHeight");
        let h = ui.number(self.menu, bar_height);
        if let Some(bar) = self.list.scrollbar {
            let own = Self::custom(ui, "_height");
            ui.set_number(bar, own, h);
            let current = Self::custom(ui, "_current_value");
            set_keeping_operators(ui, bar, current, 0.0);
        }
        self.list.refresh(ui);
        self.topics_at = Some(now);
        if self.state != State::ZoomIn {
            self.state = State::Topics;
        }
        ui.refresh();
    }

    /// The conversation is over: zoom out, then close (`00762160`).
    pub fn end(&mut self) {
        if self.state != State::Closing {
            self.state = State::Closing;
            self.zoom_time = 0.0;
        }
    }

    /// A service menu (barter, recipes, ...) opens over the conversation
    /// (`00763ff0`): unless it is ending (`+0x2c`; here zooming out), the
    /// menu is marked in service, the line being said is cut short
    /// ([`DialogMenu::cut_line`]) and the menu fades out, staying open
    /// (`Menu::StartFadeOut`, `00a1d910`, on the interface's fades as every
    /// menu's: [`crate::fade`]; `dt` the frame's seconds). True when it did.
    pub fn service_opened(&mut self, ui: &mut Ui, fades: &mut Fades, dt: f32) -> bool {
        if self.state == State::Closing || self.closed {
            return false;
        }
        self.in_service = true;
        if self.state == State::Line {
            self.cut_line = true;
        }
        fades.start_fade_out(ui, self.menu, dt);
        true
    }

    /// The service menu closed (`007640a0`): no longer in service, and the
    /// menu fades back in (`Menu::StartFadeIn`, `00a1db20`).
    pub fn service_closed(&mut self, ui: &mut Ui, fades: &mut Fades, dt: f32) {
        if !self.in_service {
            return;
        }
        self.in_service = false;
        fades.start_fade_in(ui, self.menu, dt);
    }

    /// Whether the menu is under a service menu (fading out or faded out:
    /// only a service menu fades it while it stays open), so nothing can
    /// be chosen in it (the input goes to the top menu shown and not
    /// closing, `00720e60`: the service menu).
    pub fn hidden(&self) -> bool {
        self.in_service
    }

    /// One frame (`00762950`): the zoom in or out, and `_DialogVisible`.
    /// `zoom_in` / `zoom_out` are the settings' seconds. Returns true when
    /// the first line, which waited for the zoom, can now be shown.
    pub fn update(&mut self, ui: &mut Ui, dt: f32, zoom_in: f32, zoom_out: f32) -> bool {
        let visible = Self::custom(ui, "_DialogVisible");
        let mut start_line = false;
        match self.state {
            State::ZoomIn => {
                if !self.wait_for_zoom && SHOW_AT <= self.zoom {
                    ui.set_number(self.menu, visible, 1.0);
                }
                self.zoom_time += dt;
                self.zoom = (self.zoom_time / zoom_in.max(f32::MIN_POSITIVE)).min(1.0);
                if self.zoom == 1.0 {
                    if self.wait_for_zoom {
                        ui.set_number(self.menu, visible, 1.0);
                        self.wait_for_zoom = false;
                        start_line = true;
                    }
                    self.zoom_time = 0.0;
                    if self.state == State::ZoomIn {
                        self.state = State::Line;
                    }
                }
            }
            State::Closing => {
                self.zoom_time += dt;
                self.zoom = 1.0 - (self.zoom_time / zoom_out.max(f32::MIN_POSITIVE)).min(1.0);
                if self.zoom == 0.0 {
                    self.closed = true;
                }
            }
            _ => {}
        }
        ui.refresh();
        start_line
    }
}

impl MenuCode for DialogMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..4).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `007624f0`.
    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, now: f64) {
        if matches!(self.state, State::ZoomIn | State::Closing) || self.hidden() {
            return;
        }
        if let Some(at) = self.topics_at {
            if (now - at) * 1000.0 < CLICK_WAIT_MS {
                return;
            }
            self.topics_at = None;
        }
        if id == -1 {
            if let Some(tile) = tile.filter(|t_| self.list.items.iter().any(|i| i.tile == *t_)) {
                let index = ui.number(tile, t::LISTINDEX) as i32;
                if index >= 0 && (index as usize) < self.list.items.len() {
                    self.answer = Some(Answer::Topic(index as usize));
                    self.state = State::Next;
                    return;
                }
            }
        }
        let showing = Self::custom(ui, "_ShowingText");
        if ui.number(self.menu, showing) != 0.0 && id == 0 {
            self.answer = Some(Answer::Skip);
            self.state = State::Next;
        }
    }

    /// `007628c0`: the A button / Enter clicks the chosen topic, or the
    /// screen while a line shows.
    fn special_key(&mut self, ui: &mut Ui, code: i32, now: f64) -> bool {
        if code != special::A {
            return false;
        }
        let showing = Self::custom(ui, "_ShowingText");
        if ui.number(self.menu, showing) == 0.0 {
            let chosen = self.list.selected;
            self.click(ui, -1, chosen, now);
        } else {
            self.click(ui, 0, None, now);
        }
        true
    }

    fn lists(&mut self) -> Vec<&mut ListBox> {
        vec![&mut self.list]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fade::State as FadeState;
    use crate::menus::test_support;

    fn opened() -> (Ui, DialogMenu) {
        let mut ui = test_support::ui();
        let mut m = DialogMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::dialog_menu(), &mut m);
        m.open(&mut ui, "Sunny Smiles", true, false);
        (ui, m)
    }

    fn topics() -> Vec<Topic> {
        vec![
            Topic {
                text: "Hello".into(),
                dim: false,
                failed: false,
            },
            Topic {
                text: "[Speech 12/25]  Help me".into(),
                dim: true,
                failed: true,
            },
        ]
    }

    /// `00762950`: the menu shows once the zoom is a tenth of the way, and
    /// a line shown meanwhile leaves it zooming (`00761a20`).
    #[test]
    fn the_menu_shows_as_the_camera_zooms_in() {
        let (mut ui, mut m) = opened();
        let visible = ui.names.lookup("_DialogVisible").unwrap();
        m.show_line(&mut ui, "Cheyenne, stay.");
        assert_eq!(m.state, State::ZoomIn);
        m.update(&mut ui, 0.1, ZOOM_IN_SECONDS, ZOOM_OUT_SECONDS);
        assert_eq!(ui.number(m.menu, visible), 0.0);
        m.update(&mut ui, 0.1, ZOOM_IN_SECONDS, ZOOM_OUT_SECONDS);
        m.update(&mut ui, 0.1, ZOOM_IN_SECONDS, ZOOM_OUT_SECONDS);
        assert_eq!(ui.number(m.menu, visible), 1.0);
        for _ in 0..20 {
            m.update(&mut ui, 0.1, ZOOM_IN_SECONDS, ZOOM_OUT_SECONDS);
        }
        assert_eq!(m.state, State::Line);
        let label = m.tiles[1].unwrap();
        assert_eq!(ui.string(label, t::STRING).as_deref(), Some("Sunny Smiles"));
        let text = m.tiles[2].unwrap();
        assert!(ui.shown(text));
        // Closing zooms out, then the menu goes.
        m.end();
        for _ in 0..6 {
            m.update(&mut ui, 0.1, ZOOM_IN_SECONDS, ZOOM_OUT_SECONDS);
        }
        assert!(m.closed);
    }

    /// `007638b0`: topics named `topic_<n>`, dimmed ones at alpha 128, the
    /// failed-check flag; `007624f0`: clicks wait 500 ms, then a topic is
    /// chosen by its place; the screen moves a line on.
    #[test]
    fn topics_and_choosing() {
        let (mut ui, mut m) = opened();
        for _ in 0..20 {
            m.update(&mut ui, 0.1, ZOOM_IN_SECONDS, ZOOM_OUT_SECONDS);
        }
        m.show_topics(&mut ui, &topics(), 10.0);
        assert_eq!(m.state, State::Topics);
        let showing = ui.names.lookup("_ShowingText").unwrap();
        assert_eq!(ui.number(m.menu, showing), 0.0);
        let first = m.list.items[0].tile;
        let second = m.list.items[1].tile;
        assert_eq!(ui.tiles[second].name, "topic_1");
        let alpha = ui.names.lookup("_line_alpha").unwrap();
        assert_eq!(ui.number(first, alpha), 255.0);
        assert_eq!(ui.number(second, alpha), 128.0);
        assert!(ui.tiles[second].failed && !ui.tiles[first].failed);
        // The list as high as its items (two here), at least 110.
        let list = m.tiles[3].unwrap();
        assert_eq!(ui.number(list, t::HEIGHT), 110.0);
        m.click(&mut ui, -1, Some(second), 10.2);
        assert_eq!(m.answer, None);
        m.click(&mut ui, -1, Some(second), 10.6);
        assert_eq!(m.answer, Some(Answer::Topic(1)));
        // A line: the screen moves it on; Enter does too.
        m.show_line(&mut ui, "Next.");
        m.answer = None;
        m.click(&mut ui, 0, None, 20.0);
        assert_eq!(m.answer, Some(Answer::Skip));
        m.answer = None;
        assert!(m.special_key(&mut ui, special::A, 21.0));
        assert_eq!(m.answer, Some(Answer::Skip));
        assert!(!m.special_key(&mut ui, special::UP, 21.0));
    }

    /// The menu opened as the screen opens every menu (`00a1dc20`: fading
    /// in) and shown by the end of the frame (`00711ea0`).
    fn shown(ui: &mut Ui, m: &DialogMenu) -> Fades {
        let mut fades = Fades::default();
        fades.show(ui, m.menu, false, 1.0);
        fades.frame(ui, &[m.menu]);
        assert_eq!(fades.state(m.menu), FadeState::Shown);
        fades
    }

    /// `00763ff0`: a service menu opening over the topics fades the menu
    /// out over `menufade` (0.25 by default) and leaves it open but hidden,
    /// taking no clicks; `007640a0` on the service menu's close fades it
    /// back in (`00a1db20`), the topics still there (`00711ea0`: alpha
    /// 1 − t out, t in). The fade is the interface's, as every menu's
    /// (`crate::fade`).
    #[test]
    fn a_service_menu_hides_the_conversation_until_it_closes() {
        let (mut ui, mut m) = opened();
        for _ in 0..20 {
            m.update(&mut ui, 0.1, ZOOM_IN_SECONDS, ZOOM_OUT_SECONDS);
        }
        m.show_topics(&mut ui, &topics(), 10.0);
        let mut fades = shown(&mut ui, &m);
        assert_eq!(ui.number(m.menu, t::MENUFADE), 0.25);
        assert!(m.service_opened(&mut ui, &mut fades, 0.0));
        assert!(m.in_service && m.hidden() && !m.cut_line);
        assert_eq!(fades.state(m.menu), FadeState::FadingOut);
        fades.update(0.125);
        assert!(fades.frame(&mut ui, &[m.menu]).is_empty());
        assert!((fades.value(m.menu) - 0.5).abs() < 1e-6);
        assert_eq!(ui.number(m.menu, t::VISIBLE), 1.0);
        fades.update(0.2);
        // Not marked to leave the stack: kept, hidden.
        assert_eq!(
            fades.frame(&mut ui, &[m.menu]),
            vec![(m.menu, crate::fade::Ended::Hidden)]
        );
        assert_eq!(fades.value(m.menu), 0.0);
        assert_eq!(fades.state(m.menu), FadeState::Hidden);
        assert_eq!(ui.number(m.menu, t::VISIBLE), 0.0);
        assert!(!m.closed);
        // No choosing while it's hidden.
        let first = m.list.items[0].tile;
        m.click(&mut ui, -1, Some(first), 20.0);
        assert_eq!(m.answer, None);
        // The service menu closes: back in, the same topics.
        m.service_closed(&mut ui, &mut fades, 0.0);
        assert!(!m.in_service && !m.hidden());
        assert_eq!(fades.state(m.menu), FadeState::FadingIn);
        fades.update(0.0625);
        fades.frame(&mut ui, &[m.menu]);
        assert!((fades.value(m.menu) - 0.25).abs() < 1e-6);
        assert_eq!(ui.number(m.menu, t::VISIBLE), 1.0);
        fades.update(0.25);
        fades.frame(&mut ui, &[m.menu]);
        assert_eq!(fades.value(m.menu), 1.0);
        assert_eq!(fades.state(m.menu), FadeState::Shown);
        assert_eq!(m.list.items.len(), 2);
        m.click(&mut ui, -1, Some(first), 21.0);
        assert_eq!(m.answer, Some(Answer::Topic(0)));
    }

    /// `00763ff0` cuts the line being said short; a conversation already
    /// ending (`+0x2c` set: zooming out here) isn't hidden.
    #[test]
    fn a_service_cuts_the_line_but_not_an_ending() {
        let (mut ui, mut m) = opened();
        for _ in 0..20 {
            m.update(&mut ui, 0.1, ZOOM_IN_SECONDS, ZOOM_OUT_SECONDS);
        }
        m.show_line(&mut ui, "Let's see what you've got.");
        let mut fades = shown(&mut ui, &m);
        assert!(m.service_opened(&mut ui, &mut fades, 0.0));
        assert!(m.cut_line);
        let (mut ui, mut m) = opened();
        let mut fades = shown(&mut ui, &m);
        m.end();
        assert!(!m.service_opened(&mut ui, &mut fades, 0.0));
        assert!(!m.in_service && !m.hidden());
        assert_eq!(fades.state(m.menu), FadeState::Shown);
    }
}
