//! In-memory network: perfect delivery, instant.

use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

use super::{Packet, Transport, TransportError};
use crate::ids::PlayerId;
use crate::proto::Channel;

type Inboxes = Rc<RefCell<BTreeMap<PlayerId, VecDeque<Packet>>>>;

/// A set of in-memory endpoints that can reach one another.
#[derive(Clone, Debug, Default)]
pub struct LoopbackNet {
    inboxes: Inboxes,
}

impl LoopbackNet {
    /// Empty network.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates (or re-creates, keeping queued packets) the endpoint for `id`.
    pub fn endpoint(&self, id: PlayerId) -> LoopbackEndpoint {
        self.inboxes.borrow_mut().entry(id).or_default();
        LoopbackEndpoint {
            id,
            inboxes: self.inboxes.clone(),
        }
    }

    /// Drops everything queued for `id` (simulates a dead connection).
    pub fn flush(&self, id: PlayerId) {
        if let Some(q) = self.inboxes.borrow_mut().get_mut(&id) {
            q.clear();
        }
    }
}

/// One peer's handle on a [`LoopbackNet`].
#[derive(Debug)]
pub struct LoopbackEndpoint {
    id: PlayerId,
    inboxes: Inboxes,
}

impl Transport for LoopbackEndpoint {
    fn local(&self) -> PlayerId {
        self.id
    }

    fn send(
        &mut self,
        to: PlayerId,
        channel: Channel,
        data: Vec<u8>,
    ) -> Result<(), TransportError> {
        if data.len() > channel.max_len() {
            return Err(TransportError::TooLarge {
                len: data.len(),
                max: channel.max_len(),
            });
        }
        let mut boxes = self.inboxes.borrow_mut();
        let q = boxes.get_mut(&to).ok_or(TransportError::UnknownPeer(to))?;
        q.push_back(Packet {
            from: self.id,
            channel,
            data,
        });
        Ok(())
    }

    fn recv(&mut self) -> Option<Packet> {
        self.inboxes.borrow_mut().get_mut(&self.id)?.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delivers_in_order() {
        let net = LoopbackNet::new();
        let mut a = net.endpoint(PlayerId(0));
        let mut b = net.endpoint(PlayerId(1));
        for i in 0..5u8 {
            a.send(PlayerId(1), Channel::ReliableOrdered, vec![i])
                .unwrap();
        }
        for i in 0..5u8 {
            let p = b.recv().unwrap();
            assert_eq!(p.data, vec![i]);
            assert_eq!(p.from, PlayerId(0));
        }
        assert!(b.recv().is_none());
        assert!(a.recv().is_none());
    }

    #[test]
    fn errors() {
        let net = LoopbackNet::new();
        let mut a = net.endpoint(PlayerId(0));
        assert_eq!(
            a.send(PlayerId(9), Channel::Unreliable, vec![]),
            Err(TransportError::UnknownPeer(PlayerId(9)))
        );
        let _b = net.endpoint(PlayerId(1));
        let big = vec![0; crate::proto::MAX_UNRELIABLE_BYTES + 1];
        assert!(matches!(
            a.send(PlayerId(1), Channel::Unreliable, big),
            Err(TransportError::TooLarge { .. })
        ));
    }

    #[test]
    fn flush_drops_queue() {
        let net = LoopbackNet::new();
        let mut a = net.endpoint(PlayerId(0));
        let mut b = net.endpoint(PlayerId(1));
        a.send(PlayerId(1), Channel::Unreliable, vec![1]).unwrap();
        net.flush(PlayerId(1));
        assert!(b.recv().is_none());
    }
}
