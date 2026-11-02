//! Test-only writer that builds synthetic saves the way the game lays them
//! out (docs/FOS_SAVES.md), so the reader is tested without real saves.

use crate::{LOCATION_TABLE_SIZE, MAGIC, MINOR_VERSION, VERSION};

/// Builds a pipe buffer: every value followed by `|`.
#[derive(Debug, Default, Clone)]
pub struct PipeWriter {
    pub buf: Vec<u8>,
}

impl PipeWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bytes(&mut self, b: &[u8]) -> &mut Self {
        self.buf.extend_from_slice(b);
        self.buf.push(b'|');
        self
    }

    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.bytes(&[v])
    }

    pub fn u16(&mut self, v: u16) -> &mut Self {
        self.bytes(&v.to_le_bytes())
    }

    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.bytes(&v.to_le_bytes())
    }

    pub fn i32(&mut self, v: i32) -> &mut Self {
        self.bytes(&v.to_le_bytes())
    }

    pub fn f32(&mut self, v: f32) -> &mut Self {
        self.bytes(&v.to_le_bytes())
    }

    pub fn f64(&mut self, v: f64) -> &mut Self {
        self.bytes(&v.to_le_bytes())
    }

    /// As `00865ff0` widens it: 1, 2 or 4 bytes with the size in the low
    /// two bits.
    pub fn vsval(&mut self, v: u32) -> &mut Self {
        if v < 0x40 {
            self.bytes(&[(v << 2) as u8])
        } else if v < 0x4000 {
            self.bytes(&((v << 2) as u16 | 1).to_le_bytes())
        } else {
            self.bytes(&(v << 2 | 2).to_le_bytes())
        }
    }

    pub fn ref_id(&mut self, r: u32) -> &mut Self {
        self.bytes(&[(r >> 16) as u8, (r >> 8) as u8, r as u8])
    }

    pub fn wstr(&mut self, s: &str) -> &mut Self {
        self.u16(s.len() as u16);
        if !s.is_empty() {
            self.bytes(s.as_bytes());
        }
        self
    }

    /// Zero values of these sizes (each with its `|`).
    pub fn zeros(&mut self, sizes: &[usize]) -> &mut Self {
        for &n in sizes {
            self.bytes(&vec![0; n]);
        }
        self
    }

    /// A pathing location with nothing set (`006def40`).
    pub fn pathing_location(&mut self) -> &mut Self {
        self.zeros(&[12])
            .ref_id(0)
            .ref_id(0)
            .ref_id(0)
            .zeros(&[4, 2, 1, 1])
    }

    /// What `MobileObject::SaveGame` writes after the reference's own data
    /// for an object with no process (`00932880`).
    pub fn mobile_tail(&mut self) -> &mut Self {
        self.zeros(&[1, 1, 1, 1, 1, 1, 1, 1, 4, 4, 1, 1])
            .ref_id(0)
            .ref_id(0)
    }

    /// The fixed part of `Actor::SaveGame` before its flagged parts
    /// (`008aaf40`).
    pub fn actor_fixed(&mut self) -> &mut Self {
        self.zeros(&[
            4, 1, 1, 1, 1, 4, 1, 4, 1, 1, 1, 1, 1, 1, 4, 4, 4, 1, 1, 1, 4, 4, 1, 1, 4, 1, 4, 1, 4,
            4, 4,
        ])
        .ref_id(0)
        .ref_id(0)
        .ref_id(0)
    }

    /// A mover standing still with no path (`009df100`; the player's
    /// `PlayerMover` adds four values).
    pub fn still_mover(&mut self, player: bool) -> &mut Self {
        self.zeros(&[2, 2, 1, 4, 1, 4, 1, 1, 12, 12, 4, 1, 1, 1, 1, 1, 4, 4, 4])
            .pathing_location()
            .ref_id(0)
            .u8(0);
        if player {
            self.zeros(&[12, 4, 4, 4]);
        }
        self
    }

    /// The player's data before `Character::SaveGame` (`009590f0`): three
    /// lists of 77 actor value modifiers and the health modifier.
    pub fn player_head(&mut self, script_values: &[(usize, f32)]) -> &mut Self {
        for list in 0..3 {
            for av in 0..77 {
                let v = if list == 1 {
                    script_values
                        .iter()
                        .find(|(i, _)| *i == av)
                        .map_or(0.0, |v| v.1)
                } else {
                    0.0
                };
                self.f32(v);
            }
        }
        self.f32(0.0)
    }

    /// The player's data after `Character::SaveGame` (`009590f0`), with
    /// these perks (refID, rank), active quest and hot keys.
    pub fn player_tail(
        &mut self,
        perks: &[(u32, u8)],
        active_quest: u32,
        hotkeys: [u32; 8],
    ) -> &mut Self {
        self.zeros(&[
            1, 1, 1, 1, 4, 4, 4, 4, 1, 1, 4, 4, 1, 4, 1, 1, 1, 1, 4, 12, 4, 4, 4, 1, 4, 4, 4, 1, 4,
            4, 1, 1, 4, 4, 4, 4, 1, 1, 1, 4, 4, 4, 4, 4, 4, 4, 4, 1, 1, 1, 1, 1, 1, 4, 4,
        ]);
        self.zeros(&[4, 4, 4, 4, 4]).zeros(&[1, 4]);
        self.ref_id(active_quest);
        for _ in 0..10 {
            self.ref_id(0);
        }
        // Topics, notes, Rock-It ammo, perceived actors.
        self.vsval(0).vsval(0).vsval(0).vsval(0);
        self.vsval(perks.len() as u32);
        for &(perk, rank) in perks {
            self.ref_id(perk).u8(rank);
        }
        // Actions, casino data, caravan cards.
        self.vsval(0).vsval(0).vsval(0).vsval(0);
        self.zeros(&[4, 4, 4, 4, 4]);
        // Quest log, objectives, effects.
        self.vsval(0).vsval(0).vsval(0);
        self.zeros(&[4, 4, 4]);
        self.vsval(0);
        self.zeros(&[1, 1, 1]);
        for h in hotkeys {
            self.u32(h);
        }
        self.vsval(0).vsval(0).vsval(0).vsval(0)
    }

    pub fn finish(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.buf)
    }
}

/// One change form to write.
#[derive(Debug, Clone)]
pub struct Form {
    pub ref_id: u32,
    pub flags: u32,
    pub save_type: u8,
    pub version: u8,
    pub data: Vec<u8>,
}

impl Form {
    pub fn new(ref_id: u32, flags: u32, save_type: u8, data: Vec<u8>) -> Self {
        Self {
            ref_id,
            flags,
            save_type,
            version: MINOR_VERSION,
            data,
        }
    }
}

/// A whole synthetic save.
#[derive(Debug, Clone)]
pub struct SaveWriter {
    pub version: u32,
    pub width: u32,
    pub height: u32,
    pub plugins: Vec<String>,
    pub global_data_1: Vec<(u32, Vec<u8>)>,
    pub forms: Vec<Form>,
    pub global_data_2: Vec<(u32, Vec<u8>)>,
    pub form_ids: Vec<u32>,
    pub worldspaces: Vec<u32>,
    pub history: Vec<String>,
}

impl Default for SaveWriter {
    fn default() -> Self {
        Self {
            version: VERSION,
            width: 4,
            height: 2,
            plugins: vec!["FalloutNV.esm".into()],
            global_data_1: Vec::new(),
            forms: Vec::new(),
            global_data_2: vec![(1000, Vec::new())],
            form_ids: Vec::new(),
            worldspaces: Vec::new(),
            history: Vec::new(),
        }
    }
}

/// Offsets of the parts in a written save, to check the reader with.
#[derive(Debug, Clone, Copy, Default)]
pub struct Offsets {
    pub location_table: usize,
    pub global_data_1: usize,
    pub change_forms: usize,
    pub global_data_2: usize,
    pub form_id_array: usize,
    pub history: usize,
}

impl SaveWriter {
    pub fn header(&self) -> Vec<u8> {
        let mut p = PipeWriter::new();
        p.u32(self.version);
        let mut language = [0u8; 64];
        language[..7].copy_from_slice(b"ENGLISH");
        p.bytes(&language)
            .u32(self.width)
            .u32(self.height)
            .u32(3)
            .wstr("Tester")
            .wstr("Drifter")
            .u32(2)
            .wstr("Goodsprings")
            .wstr("001.02.03");
        p.finish()
    }

    pub fn build(&self) -> (Vec<u8>, Offsets) {
        let mut out = MAGIC.to_vec();
        let header = self.header();
        out.extend_from_slice(&(header.len() as u32).to_le_bytes());
        out.extend_from_slice(&header);
        out.extend((0..self.width * self.height * 3).map(|i| i as u8));
        out.push(MINOR_VERSION);
        let mut p = PipeWriter::new();
        p.u8(self.plugins.len() as u8);
        for plugin in &self.plugins {
            p.wstr(plugin);
        }
        let plugins = p.finish();
        out.extend_from_slice(&(plugins.len() as u32).to_le_bytes());
        out.extend_from_slice(&plugins);

        let mut at = Offsets {
            location_table: out.len(),
            ..Offsets::default()
        };
        out.extend_from_slice(&[0; LOCATION_TABLE_SIZE]);
        let global = |out: &mut Vec<u8>, entries: &[(u32, Vec<u8>)]| {
            for (kind, data) in entries {
                out.extend_from_slice(&kind.to_le_bytes());
                out.extend_from_slice(&(data.len() as u32).to_le_bytes());
                out.extend_from_slice(data);
            }
        };
        at.global_data_1 = out.len();
        global(&mut out, &self.global_data_1);
        at.change_forms = out.len();
        for f in &self.forms {
            out.extend_from_slice(&[
                (f.ref_id >> 16) as u8,
                (f.ref_id >> 8) as u8,
                f.ref_id as u8,
            ]);
            out.extend_from_slice(&f.flags.to_le_bytes());
            let n = f.data.len();
            let code: u8 = if n < 0x100 {
                0
            } else if n < 0x1_0000 {
                1
            } else {
                2
            };
            out.push(f.save_type | code << 6);
            out.push(f.version);
            match code {
                0 => out.push(n as u8),
                1 => out.extend_from_slice(&(n as u16).to_le_bytes()),
                _ => out.extend_from_slice(&(n as u32).to_le_bytes()),
            }
            out.extend_from_slice(&f.data);
        }
        at.global_data_2 = out.len();
        global(&mut out, &self.global_data_2);
        at.form_id_array = out.len();
        for array in [&self.form_ids, &self.worldspaces] {
            out.extend_from_slice(&(array.len() as u32).to_le_bytes());
            for id in array {
                out.extend_from_slice(&id.to_le_bytes());
            }
        }
        at.history = out.len();
        let mut p = PipeWriter::new();
        p.u32(self.history.len() as u32);
        for h in &self.history {
            p.wstr(h);
        }
        let history = p.finish();
        out.extend_from_slice(&(history.len() as u32).to_le_bytes());
        out.extend_from_slice(&history);

        let words = [
            at.form_id_array as u32,
            at.history as u32,
            at.global_data_1 as u32,
            at.change_forms as u32,
            at.global_data_2 as u32,
            self.global_data_1.len() as u32,
            self.global_data_2.len() as u32,
            self.forms.len() as u32,
        ];
        for (i, w) in words.iter().enumerate() {
            let o = at.location_table + i * 4;
            out[o..o + 4].copy_from_slice(&w.to_le_bytes());
        }
        (out, at)
    }
}
