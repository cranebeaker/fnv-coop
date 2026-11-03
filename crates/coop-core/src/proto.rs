//! Wire messages, channels and size limits. Encoding is postcard.

use serde::{Deserialize, Serialize};

use crate::actors::ActorMsg;
use crate::avatar::AvatarMsg;
use crate::chunk::Chunk;
use crate::combat::CombatMsg;
use crate::delta::Delta;
use crate::dialogue::DialogueMsg;
use crate::error::{Error, Result};
use crate::hash::Repair;
use crate::ids::{Epoch, FormId, PlayerId, Rev, Seq, Tick};
use crate::interact::InteractMsg;
use crate::loot::LootMsg;
use crate::rules::RulesMsg;
use crate::scene::SceneMsg;
use crate::schema::Field;
use crate::social::SocialMsg;

/// Wire protocol version; bump on any incompatible message change.
pub const PROTOCOL_VERSION: u16 = 6;
/// Largest datagram on the unreliable channel (fits a typical MTU).
pub const MAX_UNRELIABLE_BYTES: usize = 1200;
/// Largest single message on the reliable channel.
pub const MAX_RELIABLE_BYTES: usize = 64 * 1024;
/// Snapshot piece size.
pub const SNAPSHOT_CHUNK_BYTES: usize = 1024;
/// Largest opaque input payload.
pub const MAX_INPUT_BYTES: usize = 256;
/// Most changes in one guest intent.
pub const MAX_INTENT_CHANGES: usize = 64;

/// Delivery class of a message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Channel {
    /// Delivered exactly once and in order.
    ReliableOrdered,
    /// Best effort, may be lost, duplicated or reordered.
    Unreliable,
}

impl Channel {
    /// Hard size limit for one message on this channel.
    pub const fn max_len(self) -> usize {
        match self {
            Channel::ReliableOrdered => MAX_RELIABLE_BYTES,
            Channel::Unreliable => MAX_UNRELIABLE_BYTES,
        }
    }
}

/// Handshake payload: everything that must match for peers to share a world.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hello {
    /// [`PROTOCOL_VERSION`].
    pub proto: u16,
    /// Hash of the build (binary identity).
    pub build_hash: u64,
    /// Hash of the sync rules / schema table.
    pub rules_hash: u64,
    /// Hash of the ordered mod list.
    pub mods_hash: u64,
    /// Game executable version text (empty: unknown).
    pub exe: String,
    /// Hash of the volatile-key set (fields that change by themselves).
    pub volatile_hash: u64,
    /// Ordered plugin file names (may be empty; then only `mods_hash` is compared).
    pub plugins: Vec<String>,
}

/// Why a handshake or resume was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RejectReason {
    /// Protocol versions differ.
    ProtocolMismatch,
    /// Builds differ.
    BuildMismatch,
    /// Rules tables differ.
    RulesMismatch,
    /// Mod lists differ.
    ModsMismatch,
    /// Game executable versions differ.
    ExeMismatch,
    /// Volatile-key sets differ.
    VolatileMismatch,
    /// Plugin sets differ (missing or extra plugins).
    PluginsMismatch,
    /// Same plugins, different load order.
    LoadOrderMismatch,
    /// Session is full.
    Full,
    /// Rejoin named a session the host does not run.
    SessionMismatch,
    /// Message made no sense in the current state.
    BadMessage,
}

/// Every message on the wire.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Msg {
    /// Guest opens the handshake.
    Hello(Hello),
    /// Host accepts the handshake.
    Welcome {
        /// Identifies this world lineage; rejoin must quote it.
        session: u64,
        /// Slot assigned to the guest.
        player: PlayerId,
        /// Host tick.
        tick: Tick,
        /// Host revision.
        rev: Rev,
    },
    /// Host refuses.
    Reject(RejectReason),
    /// Host: its own identity, sent with a refusal (or a warning when the
    /// developer override let the guest in) so the guest can name the
    /// differences.
    Incompat(Hello),
    /// Guest wants a fresh start (full snapshot).
    Join,
    /// Guest resumes with state it already holds.
    Rejoin {
        /// Session from the earlier Welcome.
        session: u64,
        /// Highest delta sequence the guest saw.
        last_seq: Seq,
        /// Guest's contiguous baseline revision (the cursor).
        have_rev: Rev,
    },
    /// Host state delta (unreliable, state based).
    Delta {
        /// Send sequence number, echoed by [`Msg::Ack`].
        seq: Seq,
        /// The delta.
        delta: Delta,
    },
    /// Guest confirms it holds everything the send `seq` contained.
    Ack {
        /// Acked send.
        seq: Seq,
    },
    /// One piece of a full snapshot (chunk id doubles as send sequence).
    Snapshot(Chunk),
    /// Guest avatar input stream (opaque to this crate).
    Input {
        /// Guest tick.
        tick: Tick,
        /// Input bytes.
        payload: Vec<u8>,
    },
    /// Guest proposes writes; host validates, orders and rebroadcasts.
    Intent {
        /// Per-guest increasing id, applied exactly once.
        id: u32,
        /// Ownership epoch the guest believes it holds.
        epoch: Epoch,
        /// Proposed writes.
        changes: Vec<(FormId, Field)>,
    },
    /// Host verdict on an intent.
    IntentAck {
        /// Intent id.
        id: u32,
        /// How many of its changes were refused.
        rejected: u16,
    },
    /// Latency probe.
    Ping {
        /// Echoed by the pong.
        nonce: u32,
    },
    /// Latency answer.
    Pong {
        /// Nonce from the ping.
        nonce: u32,
    },
    /// Host root hash at a revision.
    Hash {
        /// Host tick.
        tick: Tick,
        /// Revision hashed.
        rev: Rev,
        /// Root hash.
        root: u64,
    },
    /// Guest reports its shard hashes for a revision whose root differed.
    RepairRequest {
        /// Revision both sides hashed.
        rev: Rev,
        /// Guest's 128 shard hashes.
        shards: Vec<u64>,
    },
    /// Host answers with the differing shards only.
    Repair(Repair),
    /// Avatar stream and parts (live layer, protocol 2).
    Avatar(AvatarMsg),
    /// Actor ownership, streams, damage and spawns (live layer).
    Actor(ActorMsg),
    /// Dialogue sessions (live layer).
    Dialogue(DialogueMsg),
    /// Container loot and drop claims (live layer).
    Loot(LootMsg),
    /// Tether, barrier, votes, revive, difficulty (live layer).
    Rules(RulesMsg),
    /// Chat, markers, voice (live layer).
    Social(SocialMsg),
    /// Scenes, movies, control locks, endings and world time (live layer).
    Scene(SceneMsg),
    /// Player-world interactions: barter, doors, locks, crafting (live layer).
    Interact(InteractMsg),
    /// Host weather and clock for guests (live layer).
    Env(crate::env::EnvMsg),
    /// Cell presence, placement and deferred operations of the roam model.
    Roam(crate::roam::RoamMsg),
    /// Combat depth: NPC damage to players, shots, blasts, VATS (live layer).
    Combat(CombatMsg),
}

impl Msg {
    /// The channel this message should travel on.
    pub fn channel(&self) -> Channel {
        match self {
            Msg::Delta { .. }
            | Msg::Ack { .. }
            | Msg::Input { .. }
            | Msg::Ping { .. }
            | Msg::Pong { .. }
            | Msg::Hash { .. } => Channel::Unreliable,
            Msg::Avatar(m) if m.is_stream() => Channel::Unreliable,
            Msg::Actor(m) if m.is_stream() => Channel::Unreliable,
            Msg::Social(m) if m.is_stream() => Channel::Unreliable,
            Msg::Env(m) if m.is_stream() => Channel::Unreliable,
            Msg::Roam(m) if m.is_stream() => Channel::Unreliable,
            _ => Channel::ReliableOrdered,
        }
    }

    /// Structural validation of a decoded message.
    pub fn is_sane(&self) -> bool {
        match self {
            Msg::Input { payload, .. } => payload.len() <= MAX_INPUT_BYTES,
            Msg::Intent { changes, .. } => {
                changes.len() <= MAX_INTENT_CHANGES && changes.iter().all(|(_, f)| f.is_sane())
            }
            Msg::Delta { delta, .. } => delta.changes.iter().all(|c| c.field.is_sane()),
            Msg::Repair(r) => r.entries.iter().all(|e| e.field.is_sane()),
            Msg::Avatar(m) => m.is_sane(),
            Msg::Actor(m) => m.is_sane(),
            Msg::Loot(m) => m.is_sane(),
            Msg::Social(m) => m.is_sane(),
            Msg::Scene(m) => m.is_sane(),
            Msg::Interact(m) => m.is_sane(),
            Msg::Hello(h) | Msg::Incompat(h) => crate::compat::hello_is_sane(h),
            Msg::Roam(m) => m.is_sane(),
            Msg::Combat(m) => m.is_sane(),
            _ => true,
        }
    }
}

/// Encodes a message, enforcing its channel's size limit.
pub fn encode(msg: &Msg) -> Result<Vec<u8>> {
    let bytes = postcard::to_allocvec(msg)?;
    let max = msg.channel().max_len();
    if bytes.len() > max {
        return Err(Error::TooLarge {
            len: bytes.len(),
            max,
        });
    }
    Ok(bytes)
}

/// Decodes a message received on `channel`, enforcing size, exact length and
/// structural sanity before returning it.
pub fn decode(channel: Channel, bytes: &[u8]) -> Result<Msg> {
    if bytes.len() > channel.max_len() {
        return Err(Error::TooLarge {
            len: bytes.len(),
            max: channel.max_len(),
        });
    }
    let (msg, rest) = postcard::take_from_bytes::<Msg>(bytes)?;
    if !rest.is_empty() {
        return Err(Error::Trailing);
    }
    if !msg.is_sane() {
        return Err(Error::Chunk("insane message"));
    }
    Ok(msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::delta::{Change, SnapEntry};
    use proptest::prelude::*;

    fn arb_field() -> impl Strategy<Value = Field> {
        prop_oneof![
            any::<u16>().prop_map(|stage| Field::QuestStage { stage }),
            any::<u32>().prop_map(|flags| Field::QuestFlags { flags }),
            (any::<u16>(), any::<u8>())
                .prop_map(|(index, state)| Field::Objective { index, state }),
            any::<u32>().prop_map(|bits| Field::Global { bits }),
            any::<bool>().prop_map(|known| Field::Topic { known }),
            (any::<u32>(), any::<i32>()).prop_map(|(i, count)| Field::ItemCount {
                item: FormId(i),
                count
            }),
            any::<bool>().prop_map(|disabled| Field::RefDisabled { disabled }),
            (any::<u16>(), any::<u64>()).prop_map(|(index, bits)| Field::ScriptVar { index, bits }),
            any::<i32>().prop_map(|value| Field::Karma { value }),
            any::<i32>().prop_map(|value| Field::Reputation { value }),
            any::<i8>().prop_map(|rank| Field::FactionRank { rank }),
            (any::<i32>(), any::<i32>(), any::<i32>()).prop_map(|(x, y, z)| Field::Position {
                x,
                y,
                z
            }),
            (any::<u16>(), any::<i32>()).prop_map(|(av, value)| Field::ActorValue { av, value }),
            (any::<u16>(), proptest::collection::vec(any::<u8>(), 0..40))
                .prop_map(|(tag, data)| Field::Custom { tag, data }),
        ]
    }

    fn arb_msg() -> impl Strategy<Value = Msg> {
        prop_oneof![
            Just(Msg::Join),
            any::<u64>().prop_map(|a| Msg::Hello(Hello {
                proto: 1,
                build_hash: a,
                rules_hash: !a,
                mods_hash: 7,
                exe: String::new(),
                volatile_hash: 0,
                plugins: vec![]
            })),
            (any::<u32>(), any::<u64>(), any::<u64>()).prop_map(|(s, a, b)| Msg::Rejoin {
                session: a,
                last_seq: Seq(s),
                have_rev: Rev(b)
            }),
            any::<u32>().prop_map(|s| Msg::Ack { seq: Seq(s) }),
            any::<u32>().prop_map(|nonce| Msg::Ping { nonce }),
            (any::<u32>(), any::<u64>(), any::<u64>()).prop_map(|(t, r, root)| Msg::Hash {
                tick: Tick(t),
                rev: Rev(r),
                root
            }),
            (
                any::<u32>(),
                proptest::collection::vec((any::<u32>(), arb_field()), 0..8),
                any::<u32>()
            )
                .prop_map(|(id, ch, e)| Msg::Intent {
                    id,
                    epoch: Epoch(e),
                    changes: ch.into_iter().map(|(f, x)| (FormId(f), x)).collect()
                }),
            (
                any::<u32>(),
                any::<u64>(),
                proptest::collection::vec((any::<u32>(), arb_field(), 1u64..50), 0..8)
            )
                .prop_map(|(s, base, ch)| Msg::Delta {
                    seq: Seq(s),
                    delta: Delta {
                        tick: Tick(1),
                        base_rev: Rev(base / 2),
                        rev: Rev(base / 2 + 60),
                        complete: base % 2 == 0,
                        changes: ch
                            .into_iter()
                            .map(|(f, field, rev_off)| Change {
                                form: FormId(f),
                                field,
                                rev_off
                            })
                            .collect()
                    }
                }),
            (
                any::<u32>(),
                proptest::collection::vec((any::<u32>(), arb_field(), any::<u64>()), 0..6)
            )
                .prop_map(|(_, es)| Msg::Repair(Repair {
                    shards: vec![1, 2],
                    rev: Rev(9),
                    entries: es
                        .into_iter()
                        .map(|(f, field, r)| SnapEntry {
                            form: FormId(f),
                            field,
                            rev: Rev(r)
                        })
                        .collect()
                })),
        ]
    }

    proptest! {
        #[test]
        fn roundtrip(m in arb_msg()) {
            let bytes = postcard::to_allocvec(&m).unwrap();
            let back = postcard::from_bytes::<Msg>(&bytes).unwrap();
            prop_assert_eq!(&back, &m);
            if bytes.len() <= m.channel().max_len() {
                prop_assert_eq!(decode(m.channel(), &encode(&m).unwrap()).unwrap(), m);
            }
        }

        #[test]
        fn garbage_never_panics(bytes in proptest::collection::vec(any::<u8>(), 0..400)) {
            let _ = decode(Channel::Unreliable, &bytes);
            let _ = decode(Channel::ReliableOrdered, &bytes);
        }
    }

    #[test]
    fn channels() {
        assert_eq!(Msg::Join.channel(), Channel::ReliableOrdered);
        assert_eq!(Msg::Ack { seq: Seq(1) }.channel(), Channel::Unreliable);
        assert_eq!(
            Msg::Intent {
                id: 1,
                epoch: Epoch(0),
                changes: vec![]
            }
            .channel(),
            Channel::ReliableOrdered
        );
    }

    #[test]
    fn size_limits_enforced() {
        let big = Msg::Input {
            tick: Tick(0),
            payload: vec![0; MAX_INPUT_BYTES + 1],
        };
        let bytes = postcard::to_allocvec(&big).unwrap();
        assert!(decode(Channel::Unreliable, &bytes).is_err()); // insane
        let huge = Msg::Delta {
            seq: Seq(0),
            delta: Delta {
                tick: Tick(0),
                base_rev: Rev(0),
                rev: Rev(500),
                complete: true,
                changes: (1..400)
                    .map(|i| Change {
                        form: FormId(i),
                        field: Field::QuestFlags { flags: 0xFFFF_FFFF },
                        rev_off: i as u64,
                    })
                    .collect(),
            },
        };
        assert!(matches!(encode(&huge), Err(Error::TooLarge { .. })));
        assert!(decode(Channel::Unreliable, &vec![0u8; MAX_UNRELIABLE_BYTES + 1]).is_err());
    }

    #[test]
    fn trailing_bytes_rejected() {
        let mut b = encode(&Msg::Join).unwrap();
        b.push(0);
        assert!(matches!(
            decode(Channel::ReliableOrdered, &b),
            Err(Error::Trailing)
        ));
    }

    #[test]
    fn oversize_intent_rejected() {
        let m = Msg::Intent {
            id: 1,
            epoch: Epoch(0),
            changes: (0..MAX_INTENT_CHANGES as u32 + 1)
                .map(|i| (FormId(i), Field::Karma { value: 1 }))
                .collect(),
        };
        let b = postcard::to_allocvec(&m).unwrap();
        assert!(decode(Channel::ReliableOrdered, &b).is_err());
    }
}
