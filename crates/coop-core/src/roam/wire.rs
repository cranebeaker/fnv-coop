//! Wire messages of the scene model.

use serde::{Deserialize, Serialize};

use super::deferred::DArg;
use super::model::Placement;
use crate::avatar::{dequant_heading, dequant_pos, quant_heading, quant_pos};
use crate::ids::{CellId, FormId, PlayerId, Seq};

/// Longest function name in a forwarded operation.
pub const MAX_FUNC: usize = 64;
/// Most arguments in a forwarded operation.
pub const MAX_ARGS: usize = 8;

/// Scene messages.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoamMsg {
    /// Host to everyone (unreliable, low rate): where a player is. This is
    /// all a peer knows of players outside the cells it has loaded.
    Presence {
        /// Player.
        player: PlayerId,
        /// Their cell.
        cell: CellId,
        /// Position, 1/16 unit fixed point.
        pos: [i32; 3],
        /// Newer wins.
        seq: Seq,
    },
    /// Host to one player: start here (late join, party warp, door follow).
    Place {
        /// Cell.
        cell: CellId,
        /// Position, 1/16 unit fixed point.
        pos: [i32; 3],
        /// Heading, full circle in 65536 steps.
        heading: u16,
    },
    /// Host to the peer that owns the cell's actors: run this command on
    /// your loaded copy of the reference (a deferred script operation).
    RefOp {
        /// Operation id.
        op: u32,
        /// Reference.
        target: FormId,
        /// Its cell.
        cell: CellId,
        /// Function name.
        func: String,
        /// Arguments.
        args: Vec<DArg>,
    },
    /// Peer to host: the operation was applied.
    RefOpDone {
        /// Operation id.
        op: u32,
    },
}

impl RoamMsg {
    /// True for the unreliable presence stream.
    pub fn is_stream(&self) -> bool {
        matches!(self, RoamMsg::Presence { .. })
    }

    /// Structural bounds.
    pub fn is_sane(&self) -> bool {
        match self {
            RoamMsg::RefOp { func, args, .. } => {
                func.len() <= MAX_FUNC
                    && args.len() <= MAX_ARGS
                    && args
                        .iter()
                        .all(|a| !matches!(a, DArg::Text(t) if t.len() > MAX_FUNC))
            }
            _ => true,
        }
    }

    /// A placement message.
    pub fn place(p: Placement) -> RoamMsg {
        RoamMsg::Place {
            cell: p.cell,
            pos: p.pos.map(quant_pos),
            heading: quant_heading(p.heading),
        }
    }

    /// The placement a [`RoamMsg::Place`] carries.
    pub fn as_placement(&self) -> Option<Placement> {
        match self {
            RoamMsg::Place { cell, pos, heading } => Some(Placement {
                cell: *cell,
                pos: pos.map(dequant_pos),
                heading: dequant_heading(*heading),
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::{decode, encode, Msg};

    #[test]
    fn messages_round_trip_and_are_bounded() {
        let ms = [
            RoamMsg::Presence {
                player: PlayerId(1),
                cell: CellId(7),
                pos: [16, 32, 48],
                seq: Seq(3),
            },
            RoamMsg::place(Placement {
                cell: CellId(9),
                pos: [100.0, 200.0, 5.0],
                heading: 1.5,
            }),
            RoamMsg::RefOp {
                op: 1,
                target: FormId(0x100),
                cell: CellId(2),
                func: "PlayGroup".into(),
                args: vec![DArg::Text("x".into()), DArg::num(1.0)],
            },
            RoamMsg::RefOpDone { op: 1 },
        ];
        for m in ms {
            let msg = Msg::Roam(m);
            let bytes = encode(&msg).unwrap();
            assert_eq!(decode(msg.channel(), &bytes).unwrap(), msg);
        }
        let big = RoamMsg::RefOp {
            op: 1,
            target: FormId(1),
            cell: CellId(1),
            func: "x".repeat(100),
            args: vec![],
        };
        assert!(!big.is_sane());
        let p = RoamMsg::place(Placement {
            cell: CellId(9),
            pos: [100.0, 200.0, 5.0],
            heading: 1.5,
        })
        .as_placement()
        .unwrap();
        assert!((p.pos[0] - 100.0).abs() < 0.1 && (p.heading - 1.5).abs() < 0.01);
    }
}
