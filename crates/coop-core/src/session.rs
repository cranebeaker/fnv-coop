//! Handshake and rejoin decisions (pure functions, no I/O).

use crate::ids::{Rev, Seq};
use crate::proto::{Hello, RejectReason};

/// Checks a guest's hello against the host's identity.
pub fn check_hello(local: &Hello, remote: &Hello) -> Result<(), RejectReason> {
    match crate::compat::decide(local, remote, Default::default()) {
        crate::compat::Verdict::Refuse(r, _) => Err(r),
        _ => Ok(()),
    }
}

/// What the host knows when a guest asks to resume.
#[derive(Clone, Copy, Debug)]
pub struct HostCursor {
    /// The host's session id.
    pub session: u64,
    /// Host's current revision.
    pub rev: Rev,
    /// Oldest revision a resume may start from (history compaction floor).
    pub min_resume: Rev,
    /// Last send sequence the host ever gave this guest.
    pub last_seq: Seq,
}

/// What the guest claims to hold.
#[derive(Clone, Copy, Debug)]
pub struct RejoinRequest {
    /// Session quoted by the guest.
    pub session: u64,
    /// Highest delta sequence the guest saw.
    pub last_seq: Seq,
    /// The guest's contiguous baseline revision.
    pub have_rev: Rev,
}

/// Verdict on a rejoin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RejoinDecision {
    /// Continue with deltas from this revision.
    Resume {
        /// Baseline to build deltas against.
        base: Rev,
    },
    /// The claim cannot be trusted or is too old: send a full snapshot.
    FullSnapshot,
    /// Different world lineage: the guest must start over.
    Reject(RejectReason),
}

/// Cursor check for a rejoining guest.
pub fn decide_rejoin(host: &HostCursor, req: &RejoinRequest) -> RejoinDecision {
    if req.session != host.session {
        return RejoinDecision::Reject(RejectReason::SessionMismatch);
    }
    // A cursor from the future means the host restored an older world.
    if req.have_rev > host.rev || req.last_seq.is_newer_than(host.last_seq) {
        return RejoinDecision::FullSnapshot;
    }
    if req.have_rev < host.min_resume {
        return RejoinDecision::FullSnapshot;
    }
    RejoinDecision::Resume { base: req.have_rev }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hello() -> Hello {
        Hello {
            proto: 1,
            build_hash: 10,
            rules_hash: 20,
            mods_hash: 30,
            exe: String::new(),
            volatile_hash: 0,
            plugins: vec![],
        }
    }

    #[test]
    fn hello_checks_each_field() {
        let l = hello();
        assert_eq!(check_hello(&l, &hello()), Ok(()));
        assert_eq!(
            check_hello(
                &l,
                &Hello {
                    proto: 2,
                    ..hello()
                }
            ),
            Err(RejectReason::ProtocolMismatch)
        );
        assert_eq!(
            check_hello(
                &l,
                &Hello {
                    build_hash: 0,
                    ..hello()
                }
            ),
            Err(RejectReason::BuildMismatch)
        );
        assert_eq!(
            check_hello(
                &l,
                &Hello {
                    rules_hash: 0,
                    ..hello()
                }
            ),
            Err(RejectReason::RulesMismatch)
        );
        assert_eq!(
            check_hello(
                &l,
                &Hello {
                    mods_hash: 0,
                    ..hello()
                }
            ),
            Err(RejectReason::ModsMismatch)
        );
    }

    fn cursor() -> HostCursor {
        HostCursor {
            session: 7,
            rev: Rev(100),
            min_resume: Rev(10),
            last_seq: Seq(50),
        }
    }

    #[test]
    fn rejoin_cases() {
        let ok = RejoinRequest {
            session: 7,
            last_seq: Seq(40),
            have_rev: Rev(80),
        };
        assert_eq!(
            decide_rejoin(&cursor(), &ok),
            RejoinDecision::Resume { base: Rev(80) }
        );
        assert_eq!(
            decide_rejoin(&cursor(), &RejoinRequest { session: 8, ..ok }),
            RejoinDecision::Reject(RejectReason::SessionMismatch)
        );
        assert_eq!(
            decide_rejoin(
                &cursor(),
                &RejoinRequest {
                    have_rev: Rev(101),
                    ..ok
                }
            ),
            RejoinDecision::FullSnapshot
        );
        assert_eq!(
            decide_rejoin(
                &cursor(),
                &RejoinRequest {
                    last_seq: Seq(51),
                    ..ok
                }
            ),
            RejoinDecision::FullSnapshot
        );
        assert_eq!(
            decide_rejoin(
                &cursor(),
                &RejoinRequest {
                    have_rev: Rev(5),
                    ..ok
                }
            ),
            RejoinDecision::FullSnapshot
        );
        // boundary values resume
        assert_eq!(
            decide_rejoin(
                &cursor(),
                &RejoinRequest {
                    have_rev: Rev(100),
                    last_seq: Seq(50),
                    session: 7
                }
            ),
            RejoinDecision::Resume { base: Rev(100) }
        );
    }
}
