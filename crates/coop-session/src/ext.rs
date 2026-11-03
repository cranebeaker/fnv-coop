//! Session-level messages that ride the same transport as coop-core's
//! `Msg` but are not part of the replicated state: the host's place, script
//! commands from guests, names and ping figures.
//!
//! A packet whose first byte is [`MARK`] is one of these. `Msg` is
//! postcard encoded and its first byte is the variant index (well under
//! 0xFE), so the two never collide. All of these travel on the reliable
//! channel.

use coop_core::ids::PlayerId;

use crate::start::{Opening, Sheet};

/// First byte of a session-level packet.
pub const MARK: u8 = 0xFE;
/// Longest string carried (bytes).
pub const MAX_STR: usize = 400;

/// Where the host's player is: what a guest loads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Place {
    /// The cell (form id).
    pub cell: u32,
    /// The worldspace when outdoors (0 when none).
    pub world: u32,
    /// Feet position when known.
    pub pos: Option<[f32; 3]>,
}

impl Place {
    /// The string `open_place` takes: a worldspace outdoors, else the cell,
    /// as hex form ids.
    pub fn query(&self) -> String {
        if self.world != 0 {
            format!("{:08X}", self.world)
        } else {
            format!("{:08X}", self.cell)
        }
    }

    /// True when outdoors.
    pub fn outdoors(&self) -> bool {
        self.world != 0
    }

    /// The identity that decides whether guests must move: interior cells
    /// by cell, exteriors by worldspace only (walking between grid squares
    /// is not a change of place).
    pub fn key(&self) -> (u32, u32) {
        if self.world != 0 {
            (self.world, 0)
        } else {
            (0, self.cell)
        }
    }
}

/// An action a guest asks the host to carry out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    /// A line of the game's script language, as the console runs it.
    Line(String),
    /// Set a quest's stage (quest editor ID, stage).
    Stage(String, u16),
}

impl Command {
    /// The script line that does it.
    pub fn as_line(&self) -> String {
        match self {
            Command::Line(l) => l.clone(),
            Command::Stage(q, n) => format!("SetStage {q} {n}"),
        }
    }
}

/// One entry of the roster the host shares.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerInfo {
    /// Slot.
    pub player: PlayerId,
    /// Display name.
    pub name: String,
    /// Round trip to the host in ms (0 for the host itself).
    pub rtt_ms: u32,
}

/// A session-level message.
#[derive(Clone, Debug, PartialEq)]
pub enum Ext {
    /// Host to guest: where to be.
    Place(Place),
    /// Guest to host: do this (resent until acknowledged).
    Cmd {
        /// Per guest increasing id.
        id: u32,
        /// The action.
        cmd: Command,
    },
    /// Host to guest: command received.
    CmdAck(u32),
    /// Guest to host: name and measured round trip.
    Info {
        /// Display name.
        name: String,
        /// Round trip in ms.
        rtt_ms: u32,
    },
    /// Host to guests: everyone connected.
    Roster(Vec<PeerInfo>),
    /// Host to guest: how far the host's own opening is (see `start`).
    Opening(Opening),
    /// Guest to host: the guest's character (resent until acknowledged).
    Character(Sheet),
    /// Host to guest: the character was stored.
    SheetAck,
    /// Party-save traffic (see `crate::party`); the payload is that module's own format.
    Party(Vec<u8>),
}

struct W(Vec<u8>);

impl W {
    fn u8(&mut self, v: u8) {
        self.0.push(v);
    }
    fn u16(&mut self, v: u16) {
        self.0.extend(v.to_le_bytes());
    }
    fn u32(&mut self, v: u32) {
        self.0.extend(v.to_le_bytes());
    }
    fn f32(&mut self, v: f32) {
        self.0.extend(v.to_le_bytes());
    }
    fn str(&mut self, s: &str) {
        let mut end = s.len().min(MAX_STR);
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        self.u16(end as u16);
        self.0.extend(&s.as_bytes()[..end]);
    }
}

struct R<'a>(&'a [u8]);

impl R<'_> {
    fn take(&mut self, n: usize) -> Option<&[u8]> {
        if self.0.len() < n {
            return None;
        }
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        Some(a)
    }
    fn u8(&mut self) -> Option<u8> {
        self.take(1).map(|b| b[0])
    }
    fn u16(&mut self) -> Option<u16> {
        self.take(2).map(|b| u16::from_le_bytes([b[0], b[1]]))
    }
    fn u32(&mut self) -> Option<u32> {
        self.take(4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
    fn f32(&mut self) -> Option<f32> {
        self.u32().map(f32::from_bits)
    }
    fn str(&mut self) -> Option<String> {
        let n = self.u16()? as usize;
        if n > MAX_STR {
            return None;
        }
        String::from_utf8(self.take(n)?.to_vec()).ok()
    }
}

impl Ext {
    /// Encodes with the leading [`MARK`].
    pub fn encode(&self) -> Vec<u8> {
        let mut w = W(vec![MARK]);
        match self {
            Ext::Place(p) => {
                w.u8(1);
                w.u32(p.cell);
                w.u32(p.world);
                match p.pos {
                    Some(v) => {
                        w.u8(1);
                        v.iter().for_each(|c| w.f32(*c));
                    }
                    None => w.u8(0),
                }
            }
            Ext::Cmd { id, cmd } => {
                w.u8(2);
                w.u32(*id);
                match cmd {
                    Command::Line(l) => {
                        w.u8(0);
                        w.str(l);
                    }
                    Command::Stage(q, n) => {
                        w.u8(1);
                        w.str(q);
                        w.u16(*n);
                    }
                }
            }
            Ext::CmdAck(id) => {
                w.u8(3);
                w.u32(*id);
            }
            Ext::Info { name, rtt_ms } => {
                w.u8(4);
                w.str(name);
                w.u32(*rtt_ms);
            }
            Ext::Party(b) => {
                w.u8(9);
                w.0.extend(b);
            }
            Ext::Roster(list) => {
                w.u8(5);
                w.u8(list.len().min(255) as u8);
                for p in list.iter().take(255) {
                    w.u8(p.player.0);
                    w.str(&p.name);
                    w.u32(p.rtt_ms);
                }
            }
            Ext::Opening(o) => {
                w.u8(6);
                w.u8(o.byte());
            }
            Ext::Character(c) => {
                w.u8(7);
                w.str(&c.name);
                w.u8(match c.female {
                    None => 0,
                    Some(false) => 1,
                    Some(true) => 2,
                });
                c.special.iter().for_each(|v| w.u8(*v));
                w.u8(c.tags.len().min(32) as u8);
                c.tags.iter().take(32).for_each(|t| w.u16(*t));
                w.u8(c.perks.len().min(64) as u8);
                c.perks.iter().take(64).for_each(|p| w.u32(*p));
            }
            Ext::SheetAck => w.u8(8),
        }
        w.0
    }

    /// Decodes a packet that starts with [`MARK`].
    pub fn decode(data: &[u8]) -> Option<Ext> {
        let (&m, rest) = data.split_first()?;
        if m != MARK {
            return None;
        }
        let mut r = R(rest);
        let e = match r.u8()? {
            1 => {
                let cell = r.u32()?;
                let world = r.u32()?;
                let pos = match r.u8()? {
                    0 => None,
                    _ => Some([r.f32()?, r.f32()?, r.f32()?]),
                };
                Ext::Place(Place { cell, world, pos })
            }
            2 => {
                let id = r.u32()?;
                let cmd = match r.u8()? {
                    0 => Command::Line(r.str()?),
                    1 => {
                        let q = r.str()?;
                        Command::Stage(q, r.u16()?)
                    }
                    _ => return None,
                };
                Ext::Cmd { id, cmd }
            }
            3 => Ext::CmdAck(r.u32()?),
            4 => Ext::Info {
                name: r.str()?,
                rtt_ms: r.u32()?,
            },
            5 => {
                let n = r.u8()?;
                let mut list = Vec::new();
                for _ in 0..n {
                    list.push(PeerInfo {
                        player: PlayerId(r.u8()?),
                        name: r.str()?,
                        rtt_ms: r.u32()?,
                    });
                }
                Ext::Roster(list)
            }
            6 => Ext::Opening(Opening::from_byte(r.u8()?)?),
            7 => {
                let name = r.str()?;
                let female = match r.u8()? {
                    0 => None,
                    1 => Some(false),
                    2 => Some(true),
                    _ => return None,
                };
                let mut special = [0u8; 7];
                for v in special.iter_mut() {
                    *v = r.u8()?;
                }
                let mut tags = Vec::new();
                for _ in 0..r.u8()? {
                    tags.push(r.u16()?);
                }
                let mut perks = Vec::new();
                for _ in 0..r.u8()? {
                    perks.push(r.u32()?);
                }
                Ext::Character(Sheet {
                    name,
                    female,
                    special,
                    tags,
                    perks,
                })
            }
            8 => Ext::SheetAck,
            9 => {
                let b = r.0.to_vec();
                r.0 = &[];
                Ext::Party(b)
            }
            _ => return None,
        };
        r.0.is_empty().then_some(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let all = [
            Ext::Place(Place {
                cell: 0x1234,
                world: 0,
                pos: None,
            }),
            Ext::Place(Place {
                cell: 5,
                world: 9,
                pos: Some([1.0, -2.5, 3.0]),
            }),
            Ext::Cmd {
                id: 7,
                cmd: Command::Line("player.additem f 1".into()),
            },
            Ext::Cmd {
                id: 8,
                cmd: Command::Stage("VCG01".into(), 20),
            },
            Ext::CmdAck(7),
            Ext::Info {
                name: "Ann".into(),
                rtt_ms: 33,
            },
            Ext::Party(vec![1, 2, 3]),
            Ext::Roster(vec![
                PeerInfo {
                    player: PlayerId(0),
                    name: "Host".into(),
                    rtt_ms: 0,
                },
                PeerInfo {
                    player: PlayerId(2),
                    name: "Bo".into(),
                    rtt_ms: 90,
                },
            ]),
            Ext::Opening(Opening::InProgress),
            Ext::Opening(Opening::Over),
            Ext::Character(Sheet {
                name: "Bo".into(),
                female: Some(true),
                special: [1, 2, 3, 4, 5, 6, 10],
                tags: vec![32, 41],
                perks: vec![0x1234, 7],
            }),
            Ext::Character(Sheet::default()),
            Ext::SheetAck,
        ];
        for e in all {
            let b = e.encode();
            assert_eq!(b[0], MARK);
            assert_eq!(Ext::decode(&b), Some(e));
        }
    }

    #[test]
    fn rejects_garbage() {
        assert_eq!(Ext::decode(&[MARK]), None);
        assert_eq!(Ext::decode(&[MARK, 99]), None);
        assert_eq!(Ext::decode(&[MARK, 3, 1, 0, 0, 0, 9]), None);
        assert_eq!(Ext::decode(&[0, 3, 1, 0, 0, 0]), None);
        assert_eq!(Ext::decode(&[MARK, 2, 1, 0, 0, 0, 0, 255, 255]), None);
    }

    #[test]
    fn place_query_and_key() {
        let i = Place {
            cell: 0xAB,
            world: 0,
            pos: None,
        };
        assert_eq!(i.query(), "000000AB");
        let o = Place {
            cell: 1,
            world: 0xCD,
            pos: None,
        };
        assert_eq!(o.query(), "000000CD");
        assert_ne!(i.key(), o.key());
    }
}
