//! A fake machine for the program's tests: the adapter's fake program with frames, a screen,
//! and the few console lines that a scenario uses.

use std::collections::BTreeMap;

use coop_engine::fake::FakeProgram;
use coop_engine::files::File;
use coop_engine::game::NameTable;
use coop_engine::gate::{Counters, Intent, Rules};
use coop_engine::{addr, Program, Ret};

use crate::machine::{Machine, Screen};

/// The sample quest: shared, with the stages 10 and 20 and the objective 10.
pub const QUEST: u32 = 0x0015_d912;
/// A quest that the bundled policy gives to each player.
pub const OWN_QUEST: u32 = 0x0010_4c1c;
/// The sample global.
pub const HOUR: u32 = 0x38;
/// A global that is not a part of the clock.
pub const VALUE: u32 = 0x0b00;
/// The sample cell.
pub const CELL: u32 = 0x0010_3df9;
/// A second cell.
pub const OTHER_CELL: u32 = 0x0010_6e6f;

/// How many frames the fake shows a loading screen for a cell.
const LOAD_FRAMES: u32 = 3;

/// The seconds of one frame of play, for the clock.
pub const FRAME_SECONDS: f32 = 0.016;

/// The names of the sample data, in the text form of [`NameTable::parse`].
pub const NAMES: &str = "P 00 FalloutNV.esm\n\
    F QUST 0015D912 VCG03\nF QUST 00104C1C VCG01\n\
    F GLOB 00000035 GameYear\nF GLOB 00000036 GameMonth\nF GLOB 00000037 GameDay\n\
    F GLOB 00000038 GameHour\nF GLOB 00000039 GameDaysPassed\nF GLOB 0000003A TimeScale\n\
    F GLOB 00000B00 SampleValue\n\
    F CELL 00103DF9 GSDocMitchellHouse\nF CELL 00106E6F GSProspectorSaloonInterior\n";

/// The name table of the sample data.
pub fn names() -> NameTable {
    NameTable::parse(NAMES).expect("the sample names")
}

/// The loaded files of the sample data.
pub fn files() -> Vec<File> {
    vec![File {
        index: 0,
        name: "FalloutNV.esm".into(),
    }]
}

/// A fake game: the fake program, a screen, and a console that knows `coc`, `SetStage` and
/// `set GLOBAL to`.
pub struct FakeMachine {
    /// The program.
    pub p: FakeProgram,
    screen: Screen,
    /// How many calls of the program the console read already.
    seen: usize,
    cells: BTreeMap<String, u32>,
    globals: BTreeMap<String, u32>,
    loading: Option<(u32, u32)>,
    /// Each console line that the program ran, in order.
    pub lines: Vec<String>,
    /// The count of frames.
    pub frames: u64,
    /// The frame count at the first frame of play.
    pub first_play: Option<u64>,
    /// The frame count when the program took its gate.
    pub gate_at: Option<u64>,
    /// How deep the adapter's bracket is.
    own: u32,
    /// The intents that the gate noted.
    intents: Vec<Intent>,
    /// The frames of play that the gate counted.
    pub gate_frames: u64,
    /// False: the game's main ended.
    pub running: bool,
}

impl FakeMachine {
    /// A game at its main menu, with the sample data.
    pub fn game() -> FakeMachine {
        let mut p = FakeProgram::new();
        p.add_quest(QUEST, &[10, 20], &[10]);
        p.add_quest(OWN_QUEST, &[5], &[]);
        p.add_global(HOUR, 12.0);
        p.add_global(VALUE, 1.0);
        p.add_clock(12.0);
        p.add_cell(CELL, addr::cell_state::ATTACHED);
        p.add_cell(OTHER_CELL, 0);
        p.add_file("FalloutNV.esm");
        p.add_player([0.0, 0.0, 0.0]);
        p.console_manager = 0x3000_0000;
        p.set_u32(0x3000_0000, 0x1234);
        FakeMachine {
            p,
            screen: Screen::Menu,
            seen: 0,
            cells: [
                ("gsdocmitchellhouse".to_string(), CELL),
                ("gsprospectorsalooninterior".to_string(), OTHER_CELL),
            ]
            .into(),
            globals: [("gamehour".to_string(), HOUR)].into(),
            loading: None,
            lines: Vec::new(),
            frames: 0,
            first_play: None,
            gate_at: None,
            own: 0,
            intents: Vec::new(),
            gate_frames: 0,
            running: true,
        }
    }

    /// Opens (`true`) or closes a menu of the game: the game's clock stands in a menu.
    /// No effect before the game plays.
    pub fn menu(&mut self, open: bool) {
        self.screen = match (self.screen, open) {
            (Screen::Play, true) => Screen::Menu,
            (Screen::Menu, false) if self.first_play.is_some() => Screen::Play,
            (screen, _) => screen,
        };
    }

    /// This guest's game wanted to make a change of a shared quest: its gate noted it.
    pub fn wants(&mut self, intent: Intent) {
        self.intents.push(intent);
    }

    /// The address of a form.
    fn form(&mut self, id: u32) -> u32 {
        self.p.call(addr::FORM_BY_ID, &[id]).eax
    }

    /// Runs one console line as the game does: the three functions that a scenario uses.
    fn run(&mut self, line: &str) {
        let words: Vec<&str> = line.split_whitespace().collect();
        let lower: Vec<String> = words.iter().map(|w| w.to_ascii_lowercase()).collect();
        match lower
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .as_slice()
        {
            ["coc", cell] => {
                if let Some(id) = self.cells.get(*cell).copied() {
                    self.loading = Some((id, LOAD_FRAMES));
                    self.screen = Screen::Loading;
                }
            }
            // The program's console takes a quest by its form id only.
            ["setstage", quest, stage] => {
                if let (Ok(id), Ok(stage)) = (u32::from_str_radix(quest, 16), stage.parse::<u32>())
                {
                    if quest.len() == 8 {
                        let at = self.form(id);
                        if at != 0 {
                            self.p.call(addr::QUEST_SET_STAGE, &[at, stage]);
                        }
                    }
                }
            }
            // The program's console takes a global by its editor id only.
            ["set", global, "to", value] => {
                if let (Some(id), Ok(value)) =
                    (self.globals.get(*global).copied(), value.parse::<f32>())
                {
                    let at = self.form(id);
                    self.p.set_f32(at + addr::GLOBAL_VALUE, value);
                }
            }
            _ => {}
        }
    }

    /// The console lines that the adapter gave the program since the last frame.
    fn console(&mut self) {
        let texts: Vec<u32> = self.p.calls[self.seen..]
            .iter()
            .filter(|(at, _)| *at == addr::SCRIPT_SET_TEXT)
            .map(|(_, args)| args[1])
            .collect();
        for text in texts {
            let mut bytes = [0u8; 200];
            self.p.read(text, &mut bytes);
            let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
            let line = String::from_utf8_lossy(&bytes[..end]).into_owned();
            self.run(&line);
            self.lines.push(line);
        }
        self.seen = self.p.calls.len();
    }
}

impl Program for FakeMachine {
    fn read(&self, addr: u32, out: &mut [u8]) {
        self.p.read(addr, out);
    }

    fn write(&mut self, addr: u32, bytes: &[u8]) {
        self.p.write(addr, bytes);
    }

    fn call(&mut self, addr: u32, args: &[u32]) -> Ret {
        self.p.call(addr, args)
    }

    fn alloc(&mut self, len: u32) -> u32 {
        self.p.alloc(len)
    }

    fn free(&mut self, addr: u32) {
        self.p.free(addr);
    }

    fn install_gate(&mut self, rules: &Rules, shared_quest: Option<u32>) -> Result<(), String> {
        self.gate_at = Some(self.frames);
        self.p.install_gate(rules, shared_quest)
    }
}

impl Machine for FakeMachine {
    fn frame(&mut self) -> bool {
        if !self.running {
            return false;
        }
        self.frames += 1;
        self.console();
        if let Some((cell, left)) = self.loading {
            if left == 0 {
                let at = self.form(cell);
                let player = self.p.u32(addr::PLAYER);
                self.p.set_u32(player + addr::REF_CELL, at);
                self.loading = None;
                self.screen = Screen::Play;
            } else {
                self.loading = Some((cell, left - 1));
            }
        }
        if self.screen == Screen::Play && self.first_play.is_none() {
            self.first_play = Some(self.frames);
        }
        // The program's main loop: one advance of the clock for each frame of play.
        if self.screen == Screen::Play {
            self.p
                .call(addr::CLOCK_ADVANCE, &[addr::CLOCK, FRAME_SECONDS.to_bits()]);
        }
        self.seen = self.p.calls.len();
        true
    }

    fn screen(&self) -> Screen {
        self.screen
    }

    fn own(&mut self, on: bool) {
        if on {
            self.own += 1;
        } else {
            self.own = self.own.saturating_sub(1);
        }
    }

    fn gate_frame(&mut self, _play_ms: u32) -> bool {
        self.gate_frames += 1;
        false
    }

    fn take_intents(&mut self) -> Vec<Intent> {
        std::mem::take(&mut self.intents)
    }

    fn counters(&self) -> Option<Counters> {
        (!self.p.gates.is_empty()).then(|| Counters {
            runs: self.gate_frames,
            ..Counters::default()
        })
    }
}
