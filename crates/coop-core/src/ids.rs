//! Identifier and counter newtypes shared by every module.

use serde::{Deserialize, Serialize};

/// A game FormID (plugin index in the top byte, as in the ESM).
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct FormId(pub u32);

/// An interior cell or exterior grid cell id (interest and ownership are per cell).
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct CellId(pub u32);

/// A player slot. Slot 0 is always the host.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PlayerId(pub u8);

impl PlayerId {
    /// The host is always player 0.
    pub const HOST: PlayerId = PlayerId(0);

    /// True for the host slot.
    pub fn is_host(self) -> bool {
        self == Self::HOST
    }
}

/// Ownership epoch. Bumped every time an entity changes owner so late
/// writes from the previous owner can be fenced off.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Epoch(pub u32);

impl Epoch {
    /// The next epoch.
    pub fn next(self) -> Epoch {
        Epoch(self.0.wrapping_add(1))
    }
}

/// Dense per-connection message sequence number (wraps, serial arithmetic).
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Seq(pub u32);

impl Seq {
    /// The next sequence number (wrapping).
    pub fn next(self) -> Seq {
        Seq(self.0.wrapping_add(1))
    }

    /// Serial-number comparison: true when `self` is strictly after `other`
    /// even across a wrap.
    pub fn is_newer_than(self, other: Seq) -> bool {
        (self.0.wrapping_sub(other.0) as i32) > 0
    }
}

/// Host simulation tick.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Tick(pub u32);

/// Host write counter. Every accepted host write gets the next revision, so a
/// revision is both a per-key version and a position in host order.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Rev(pub u64);

impl Rev {
    /// Revision zero: nothing written yet.
    pub const ZERO: Rev = Rev(0);

    /// The next revision.
    pub fn next(self) -> Rev {
        Rev(self.0 + 1)
    }
}

/// Number of hash shards the FormID space is split into.
pub const SHARD_COUNT: usize = 128;

/// Maps a FormID to its hash shard (0..128). A multiplicative mix keeps the
/// distribution even although FormIDs cluster by plugin index.
pub fn shard_of(form: FormId) -> u8 {
    (form.0.wrapping_mul(0x9E37_79B1) >> 25) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq_serial_arithmetic() {
        assert!(Seq(5).is_newer_than(Seq(4)));
        assert!(!Seq(4).is_newer_than(Seq(5)));
        assert!(!Seq(4).is_newer_than(Seq(4)));
        assert!(Seq(2).is_newer_than(Seq(u32::MAX - 2)));
        assert_eq!(Seq(u32::MAX).next(), Seq(0));
    }

    #[test]
    fn host_is_zero() {
        assert!(PlayerId::HOST.is_host());
        assert!(!PlayerId(1).is_host());
    }

    #[test]
    fn shards_in_range_and_spread() {
        let mut counts = [0u32; SHARD_COUNT];
        for i in 0..20_000u32 {
            let s = shard_of(FormId(0x0100_0000 + i)) as usize;
            assert!(s < SHARD_COUNT);
            counts[s] += 1;
        }
        // every shard used, none wildly overloaded (mean is ~156)
        assert!(counts.iter().all(|&c| c > 50 && c < 400));
    }

    #[test]
    fn rev_next() {
        assert_eq!(Rev::ZERO.next(), Rev(1));
        assert_eq!(Epoch(1).next(), Epoch(2));
    }
}
