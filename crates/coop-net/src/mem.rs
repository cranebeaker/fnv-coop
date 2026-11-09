//! In-process backend: lossless queues between a host and guests. Used by the
//! conformance tests and handy for single-process integration tests. Not a
//! network; `silence` simulates a peer that stops talking.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

use crate::backend::{Backend, BackendError, BackendEvent, Lane, PeerKey};

#[derive(Default)]
struct Net {
    inbox: HashMap<PeerKey, VecDeque<BackendEvent>>,
    silenced: HashSet<PeerKey>,
    next: PeerKey,
}

/// A shared in-process network.
#[derive(Clone, Default)]
pub struct MemNet(Arc<Mutex<Net>>);

/// One node of a [`MemNet`]. The host has id 0; a guest's host peer is 0 and
/// the host sees each guest under the guest's id.
pub struct MemBackend {
    net: MemNet,
    id: PeerKey,
    is_guest: bool,
}

impl MemNet {
    /// A new empty network.
    pub fn new() -> Self {
        Self::default()
    }
    /// The host node.
    pub fn host(&self) -> MemBackend {
        let mut n = self.0.lock().unwrap();
        n.inbox.entry(0).or_default();
        n.next = n.next.max(1);
        MemBackend {
            net: self.clone(),
            id: 0,
            is_guest: false,
        }
    }
    /// A new guest node.
    pub fn guest(&self) -> MemBackend {
        let mut n = self.0.lock().unwrap();
        let id = n.next.max(1);
        n.next = id + 1;
        n.inbox.entry(id).or_default();
        MemBackend {
            net: self.clone(),
            id,
            is_guest: true,
        }
    }
    /// Makes node `id` stop sending (it still receives).
    pub fn silence(&self, id: PeerKey, on: bool) {
        let mut n = self.0.lock().unwrap();
        if on {
            n.silenced.insert(id);
        } else {
            n.silenced.remove(&id);
        }
    }
}

impl MemBackend {
    /// This node's id.
    pub fn id(&self) -> PeerKey {
        self.id
    }
}

impl Backend for MemBackend {
    fn send(&mut self, peer: PeerKey, lane: Lane, data: &[u8]) -> Result<(), BackendError> {
        let mut n = self.net.0.lock().unwrap();
        if n.silenced.contains(&self.id) {
            return Ok(());
        }
        match n.inbox.get_mut(&peer) {
            Some(q) => {
                q.push_back(BackendEvent::Frame {
                    peer: self.id,
                    lane,
                    data: data.to_vec(),
                });
                Ok(())
            }
            None => Err(BackendError::UnknownPeer(peer)),
        }
    }

    fn poll(&mut self, _now_ms: u64) -> Vec<BackendEvent> {
        let mut n = self.net.0.lock().unwrap();
        n.inbox
            .get_mut(&self.id)
            .map(|q| q.drain(..).collect())
            .unwrap_or_default()
    }

    fn host_peer(&self) -> Option<PeerKey> {
        self.is_guest.then_some(0)
    }

    fn disconnect(&mut self, _peer: PeerKey) {}
}
