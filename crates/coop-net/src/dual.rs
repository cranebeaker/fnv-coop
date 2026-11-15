//! steam: two backends under one host [`crate::Endpoint`], and a boxed backend.
//!
//! A host on Steam still gives a join code that works without Steam: [`Dual`] carries the UDP
//! backend's peers and the Steam lobby's at once, so guests of both kinds join the same session.
//! The first backend's peer keys are marked with [`FIRST_MARK`] (bit 63) on the way up and
//! unmarked on the way down; a SteamID64 never has that bit (its top byte is the universe, 1 for
//! the public one), and a UDP backend's keys are small table indices.
//!
//! `Box<dyn Backend>` is a backend too, so a program picks UDP, Steam or both at run time and
//! keeps one endpoint type.

use crate::backend::{Backend, BackendError, BackendEvent, Lane, PeerKey, Readiness};

/// The mark on the first backend's peer keys.
pub const FIRST_MARK: PeerKey = 1 << 63;

/// Two host backends as one. Host only: a guest talks to one host over one backend.
pub struct Dual<A: Backend, B: Backend> {
    /// The backend whose keys are marked (UDP).
    pub first: A,
    /// The backend whose keys pass unchanged (Steam).
    pub second: B,
    /// Events of the second backend dropped because their key had the mark.
    pub clashes: u64,
}

impl<A: Backend, B: Backend> Dual<A, B> {
    /// Both backends, hosting.
    pub fn new(first: A, second: B) -> Self {
        Dual {
            first,
            second,
            clashes: 0,
        }
    }
}

impl<A: Backend, B: Backend> Backend for Dual<A, B> {
    fn send(&mut self, peer: PeerKey, lane: Lane, data: &[u8]) -> Result<(), BackendError> {
        if peer & FIRST_MARK != 0 {
            self.first
                .send(peer & !FIRST_MARK, lane, data)
                .map_err(|e| match e {
                    BackendError::UnknownPeer(k) => BackendError::UnknownPeer(k | FIRST_MARK),
                    other => other,
                })
        } else {
            self.second.send(peer, lane, data)
        }
    }

    fn poll(&mut self, now_ms: u64) -> Vec<BackendEvent> {
        let mut out: Vec<BackendEvent> = self
            .first
            .poll(now_ms)
            .into_iter()
            .map(|ev| match ev {
                BackendEvent::Frame { peer, lane, data } => BackendEvent::Frame {
                    peer: peer | FIRST_MARK,
                    lane,
                    data,
                },
                BackendEvent::PeerGone { peer } => BackendEvent::PeerGone {
                    peer: peer | FIRST_MARK,
                },
            })
            .collect();
        for ev in self.second.poll(now_ms) {
            let key = match &ev {
                BackendEvent::Frame { peer, .. } | BackendEvent::PeerGone { peer } => *peer,
            };
            if key & FIRST_MARK != 0 {
                self.clashes += 1;
                continue;
            }
            out.push(ev);
        }
        out
    }

    /// Ready when either is: a Steam lobby still being made (or one that failed) leaves the UDP
    /// side hosting.
    fn readiness(&self) -> Readiness {
        match (self.first.readiness(), self.second.readiness()) {
            (Readiness::Ready, _) | (_, Readiness::Ready) => Readiness::Ready,
            (Readiness::Failed(a), Readiness::Failed(b)) => Readiness::Failed(format!("{a}; {b}")),
            _ => Readiness::Pending,
        }
    }

    fn host_peer(&self) -> Option<PeerKey> {
        None
    }

    fn disconnect(&mut self, peer: PeerKey) {
        if peer & FIRST_MARK != 0 {
            self.first.disconnect(peer & !FIRST_MARK);
        } else {
            self.second.disconnect(peer);
        }
    }

    fn rtt_ms(&self, peer: PeerKey) -> Option<u32> {
        if peer & FIRST_MARK != 0 {
            self.first.rtt_ms(peer & !FIRST_MARK)
        } else {
            self.second.rtt_ms(peer)
        }
    }
}

impl<B: Backend + ?Sized> Backend for Box<B> {
    fn send(&mut self, peer: PeerKey, lane: Lane, data: &[u8]) -> Result<(), BackendError> {
        (**self).send(peer, lane, data)
    }
    fn poll(&mut self, now_ms: u64) -> Vec<BackendEvent> {
        (**self).poll(now_ms)
    }
    fn readiness(&self) -> Readiness {
        (**self).readiness()
    }
    fn host_peer(&self) -> Option<PeerKey> {
        (**self).host_peer()
    }
    fn disconnect(&mut self, peer: PeerKey) {
        (**self).disconnect(peer)
    }
    fn rtt_ms(&self, peer: PeerKey) -> Option<u32> {
        (**self).rtt_ms(peer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoint::{Endpoint, EndpointConfig, NetEvent};
    use crate::mem::MemNet;
    use coop_core::ids::PlayerId;
    use coop_core::proto::Channel;
    use coop_core::transport::Transport;

    const TOKEN: [u8; 16] = [3; 16];

    fn cfg(name: &str) -> EndpointConfig {
        EndpointConfig {
            name: name.into(),
            ..EndpointConfig::default()
        }
    }

    #[test]
    fn guests_of_both_backends_join_one_host_and_reach_it() {
        let a = MemNet::new();
        let b = MemNet::new();
        let dual: Box<dyn Backend> = Box::new(Dual::new(a.host(), b.host()));
        let mut host = Endpoint::host(dual, TOKEN, cfg("host"));
        let mut ga = Endpoint::guest(Box::new(a.guest()) as Box<dyn Backend>, TOKEN, cfg("a"));
        let mut gb = Endpoint::guest(b.guest(), TOKEN, cfg("b"));
        for _ in 0..20 {
            host.pump();
            ga.pump();
            gb.pump();
        }
        let joined: Vec<String> = host
            .events()
            .into_iter()
            .filter_map(|e| match e {
                NetEvent::PeerJoined { name, .. } => Some(name),
                _ => None,
            })
            .collect();
        assert_eq!(joined.len(), 2, "{joined:?}");
        assert!(joined.contains(&"a".to_string()) && joined.contains(&"b".to_string()));
        let (pa, pb) = (ga.local(), gb.local());
        assert_ne!(pa, pb);
        // Data both ways, to each guest by its own backend.
        ga.send(PlayerId::HOST, Channel::ReliableOrdered, vec![1])
            .unwrap();
        gb.send(PlayerId::HOST, Channel::Unreliable, vec![2])
            .unwrap();
        host.send(pa, Channel::ReliableOrdered, vec![10]).unwrap();
        host.send(pb, Channel::ReliableOrdered, vec![20]).unwrap();
        for _ in 0..5 {
            host.pump();
            ga.pump();
            gb.pump();
        }
        let mut got = Vec::new();
        while let Some(p) = host.recv() {
            got.push((p.from, p.data));
        }
        got.sort();
        let mut want = vec![(pa, vec![1]), (pb, vec![2])];
        want.sort();
        assert_eq!(got, want);
        assert_eq!(ga.recv().map(|p| p.data), Some(vec![10]));
        assert_eq!(gb.recv().map(|p| p.data), Some(vec![20]));
    }

    #[test]
    fn keys_are_marked_and_unmarked_and_a_clash_is_dropped() {
        struct Fixed(Vec<BackendEvent>, Vec<PeerKey>);
        impl Backend for Fixed {
            fn send(&mut self, peer: PeerKey, _: Lane, _: &[u8]) -> Result<(), BackendError> {
                self.1.push(peer);
                Ok(())
            }
            fn poll(&mut self, _: u64) -> Vec<BackendEvent> {
                std::mem::take(&mut self.0)
            }
            fn host_peer(&self) -> Option<PeerKey> {
                None
            }
            fn disconnect(&mut self, _: PeerKey) {}
        }
        let steam_id: PeerKey = 0x0110_0001_0000_0002;
        let mut d = Dual::new(
            Fixed(vec![BackendEvent::PeerGone { peer: 4 }], vec![]),
            Fixed(
                vec![
                    BackendEvent::PeerGone { peer: steam_id },
                    BackendEvent::PeerGone {
                        peer: FIRST_MARK | 1,
                    },
                ],
                vec![],
            ),
        );
        assert_eq!(
            d.poll(0),
            vec![
                BackendEvent::PeerGone {
                    peer: 4 | FIRST_MARK
                },
                BackendEvent::PeerGone { peer: steam_id }
            ]
        );
        assert_eq!(d.clashes, 1);
        d.send(4 | FIRST_MARK, Lane::Reliable, &[]).unwrap();
        d.send(steam_id, Lane::Reliable, &[]).unwrap();
        assert_eq!(d.first.1, vec![4]);
        assert_eq!(d.second.1, vec![steam_id]);
        assert_eq!(d.host_peer(), None);
    }

    #[test]
    fn ready_while_either_side_is() {
        struct R(Readiness);
        impl Backend for R {
            fn send(&mut self, _: PeerKey, _: Lane, _: &[u8]) -> Result<(), BackendError> {
                Ok(())
            }
            fn poll(&mut self, _: u64) -> Vec<BackendEvent> {
                Vec::new()
            }
            fn readiness(&self) -> Readiness {
                self.0.clone()
            }
            fn host_peer(&self) -> Option<PeerKey> {
                None
            }
            fn disconnect(&mut self, _: PeerKey) {}
        }
        let d = |a, b| Dual::new(R(a), R(b)).readiness();
        assert_eq!(
            d(Readiness::Ready, Readiness::Failed("x".into())),
            Readiness::Ready
        );
        assert_eq!(d(Readiness::Pending, Readiness::Ready), Readiness::Ready);
        assert_eq!(
            d(Readiness::Pending, Readiness::Pending),
            Readiness::Pending
        );
        assert_eq!(
            d(Readiness::Failed("a".into()), Readiness::Failed("b".into())),
            Readiness::Failed("a; b".into())
        );
    }
}
