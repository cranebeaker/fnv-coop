//! Control-lane messages and the keep-alive timers.
//!
//! Neither Steam Messages nor raw UDP tells you a peer went away, so every
//! backend gets the same rule: a keep-alive every second, a peer silent for
//! twenty seconds is dropped, goodbye and kick are explicit.

use serde::{Deserialize, Serialize};

/// Keep-alive interval.
pub const KEEPALIVE_MS: u64 = 1_000;
/// Silence after which a peer is dropped.
pub const TIMEOUT_MS: u64 = 20_000;
/// Wire protocol version of this crate's control messages.
pub const NET_PROTO: u16 = 1;

/// Size of the session token.
pub const TOKEN_LEN: usize = 16;

/// Why the host refused a join.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RejectCode {
    /// Wrong session token.
    BadToken,
    /// No free slot.
    Full,
    /// Control protocol versions differ.
    ProtoMismatch,
}

/// Control-lane messages.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ctl {
    /// Guest asks to join.
    Join {
        /// Control protocol version.
        proto: u16,
        /// Session token from the join code.
        token: [u8; TOKEN_LEN],
        /// Display name.
        name: String,
    },
    /// Host accepted; the guest is player `slot`.
    Welcome {
        /// Assigned slot (1..).
        slot: u8,
    },
    /// Host refused.
    Reject(RejectCode),
    /// Periodic liveness ping, either direction.
    KeepAlive,
    /// Clean leave, either direction.
    Goodbye,
    /// Host removes a guest.
    Kick {
        /// Shown to the guest.
        reason: String,
    },
}

/// Per-peer liveness timers (pure, driven by a ms clock).
#[derive(Clone, Debug)]
pub struct Liveness {
    last_rx: u64,
    last_keepalive: u64,
}

impl Liveness {
    /// Starts counting at `now`.
    pub fn new(now: u64) -> Self {
        Liveness {
            last_rx: now,
            last_keepalive: now,
        }
    }
    /// Anything arrived from the peer.
    pub fn on_rx(&mut self, now: u64) {
        self.last_rx = now;
    }
    /// True once per interval when a keep-alive should be sent (and marks it sent).
    pub fn keepalive_due(&mut self, now: u64) -> bool {
        if now.saturating_sub(self.last_keepalive) >= KEEPALIVE_MS {
            self.last_keepalive = now;
            true
        } else {
            false
        }
    }
    /// True if the peer has been silent too long.
    pub fn timed_out(&self, now: u64) -> bool {
        now.saturating_sub(self.last_rx) >= TIMEOUT_MS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn keepalive_cadence_and_timeout() {
        let mut l = Liveness::new(0);
        assert!(!l.keepalive_due(999));
        assert!(l.keepalive_due(1000));
        assert!(!l.keepalive_due(1500));
        assert!(l.keepalive_due(2000));
        assert!(!l.timed_out(19_999));
        assert!(l.timed_out(20_000));
        l.on_rx(19_000);
        assert!(!l.timed_out(20_000));
        assert!(l.timed_out(39_000));
    }

    proptest! {
        #[test]
        fn ctl_roundtrip(name in ".{0,40}", slot in any::<u8>(), tok in any::<[u8; 16]>(), reason in ".{0,60}") {
            for m in [
                Ctl::Join { proto: NET_PROTO, token: tok, name: name.clone() },
                Ctl::Welcome { slot },
                Ctl::Reject(RejectCode::Full),
                Ctl::KeepAlive,
                Ctl::Goodbye,
                Ctl::Kick { reason: reason.clone() },
            ] {
                let b = postcard::to_stdvec(&m).unwrap();
                prop_assert_eq!(postcard::from_bytes::<Ctl>(&b).unwrap(), m);
            }
        }

        #[test]
        fn ctl_garbage_never_panics(b in proptest::collection::vec(any::<u8>(), 0..100)) {
            let _ = postcard::from_bytes::<Ctl>(&b);
        }
    }
}
