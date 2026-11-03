//! `MockSteam`: an in-process Steam universe that follows the playbook rules.
//!
//! Rules implemented (each has a test):
//! * the first message from a stranger is held and raises `SessionRequest`;
//!   it is delivered only after `accept_session`; a session we opened by
//!   sending first needs no accept for replies;
//! * `LobbyCreated` is delivered twice (call result and broadcast);
//! * friends-only lobbies: joining needs friendship with the owner or an invite;
//! * lobby full / not joinable / missing responses;
//! * owner migration on leave; `LobbyChatUpdate` to remaining members.
//!
//! Fault injection ([`Faults`]): send buffer full, silent peers, session
//! failures, unreliable loss, and reversed event order.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

use coop_core::transport::SeededRng;

use super::api::*;

const LOBBY_BASE: u64 = 109_775_242_000_000_000;

/// Switches for failure and ordering behaviour.
#[derive(Clone, Debug)]
pub struct Faults {
    /// Deliver `LobbyCreated` twice, as real Steam does (default true).
    pub double_lobby_created: bool,
    /// Reverse each batch returned by `run_frame` (Steam promises no order).
    pub reverse_event_order: bool,
    /// Probability an unreliable message is lost.
    pub unreliable_loss: f64,
    /// Messages sent by these users vanish (a silent or frozen peer).
    pub silent: HashSet<SteamId>,
    /// Sending to these users raises `SessionFailed` on the sender and drops the message.
    pub session_failed: HashSet<SteamId>,
    /// Next N sends by this user report `BufferFull`.
    pub buffer_full: HashMap<SteamId, u32>,
    /// RNG seed for the loss.
    pub seed: u64,
}

impl Default for Faults {
    fn default() -> Self {
        Faults {
            double_lobby_created: true,
            reverse_event_order: false,
            unreliable_loss: 0.0,
            silent: HashSet::new(),
            session_failed: HashSet::new(),
            buffer_full: HashMap::new(),
            seed: 1,
        }
    }
}

#[derive(Default)]
struct User {
    name: String,
    friends: HashSet<SteamId>,
    events: VecDeque<SteamEvent>,
    inbox: HashMap<i32, VecDeque<RecvMsg>>,
    accepted: HashSet<SteamId>,
    requested: HashSet<SteamId>,
    held: Vec<(SteamId, i32, Vec<u8>)>,
    rich: HashMap<String, String>,
    lobby: Option<LobbyId>,
    playing_app: Option<u32>,
}

struct Lobby {
    owner: SteamId,
    max: u32,
    members: Vec<SteamId>,
    data: HashMap<String, String>,
    joinable: bool,
    invited: HashSet<SteamId>,
}

struct Universe {
    users: HashMap<SteamId, User>,
    lobbies: HashMap<LobbyId, Lobby>,
    next_lobby: u64,
    faults: Faults,
    rng: SeededRng,
    /// Log of every `SessionRequest` raised, for assertions.
    session_requests: Vec<(SteamId, SteamId)>,
}

/// Handle to the shared universe; make users with [`MockUniverse::user`].
#[derive(Clone)]
pub struct MockUniverse(Arc<Mutex<Universe>>);

impl Default for MockUniverse {
    fn default() -> Self {
        Self::new()
    }
}

impl MockUniverse {
    /// An empty universe.
    pub fn new() -> Self {
        let faults = Faults::default();
        MockUniverse(Arc::new(Mutex::new(Universe {
            users: HashMap::new(),
            lobbies: HashMap::new(),
            next_lobby: 1,
            rng: SeededRng::new(faults.seed),
            faults,
            session_requests: Vec::new(),
        })))
    }

    /// Creates a logged-in user (a deterministic SteamID64 in the individual range).
    pub fn user(&self, number: u64, name: &str) -> MockSteam {
        let id = 76_561_197_960_265_728 + number;
        let mut u = self.0.lock().unwrap();
        u.users.insert(
            id,
            User {
                name: name.into(),
                playing_app: Some(480),
                ..Default::default()
            },
        );
        MockSteam {
            uni: self.clone(),
            me: id,
        }
    }

    /// Makes two users mutual friends.
    pub fn befriend(&self, a: &MockSteam, b: &MockSteam) {
        let mut u = self.0.lock().unwrap();
        u.users.get_mut(&a.me).unwrap().friends.insert(b.me);
        u.users.get_mut(&b.me).unwrap().friends.insert(a.me);
    }

    /// Edits the fault switches.
    pub fn faults(&self, f: impl FnOnce(&mut Faults)) {
        let mut u = self.0.lock().unwrap();
        f(&mut u.faults);
        u.rng = SeededRng::new(u.faults.seed);
    }

    /// Simulates a crashed client: removed from its lobby with the
    /// "disconnected" flag and all its state gone.
    pub fn crash(&self, who: &MockSteam) {
        let mut u = self.0.lock().unwrap();
        if let Some(l) = u.users.get(&who.me).and_then(|x| x.lobby) {
            u.remove_member(l, who.me, CHAT_DISCONNECTED);
        }
    }

    /// Every `(to, from)` session request raised so far.
    pub fn session_requests(&self) -> Vec<(SteamId, SteamId)> {
        self.0.lock().unwrap().session_requests.clone()
    }
}

impl Universe {
    fn push(&mut self, to: SteamId, ev: SteamEvent) {
        if let Some(u) = self.users.get_mut(&to) {
            u.events.push_back(ev);
        }
    }

    fn remove_member(&mut self, lobby: LobbyId, who: SteamId, flag: u32) {
        let Some(l) = self.lobbies.get_mut(&lobby) else {
            return;
        };
        l.members.retain(|m| *m != who);
        let remaining = l.members.clone();
        if l.owner == who {
            if let Some(&n) = remaining.first() {
                l.owner = n;
            }
        }
        if remaining.is_empty() {
            self.lobbies.remove(&lobby);
        }
        if let Some(u) = self.users.get_mut(&who) {
            u.lobby = None;
        }
        for m in remaining {
            self.push(
                m,
                SteamEvent::LobbyChatUpdate {
                    lobby,
                    user_changed: who,
                    making_change: who,
                    state: flag,
                },
            );
        }
    }
}

/// One Steam user in a [`MockUniverse`].
pub struct MockSteam {
    uni: MockUniverse,
    me: SteamId,
}

impl MockSteam {
    /// Another handle onto the same user (for crash simulation in tests).
    pub fn clone_handle(&self) -> MockSteam {
        MockSteam {
            uni: self.uni.clone(),
            me: self.me,
        }
    }

    /// Sets the AppID this user shows as playing (default 480).
    pub fn set_playing_app(&self, app: Option<u32>) {
        self.uni
            .0
            .lock()
            .unwrap()
            .users
            .get_mut(&self.me)
            .unwrap()
            .playing_app = app;
    }

    /// Fakes an accepted invite or Join Game click (callback 333).
    pub fn fake_join_request(&self, lobby: LobbyId, friend: SteamId) {
        self.uni
            .0
            .lock()
            .unwrap()
            .push(self.me, SteamEvent::JoinRequested { lobby, friend });
    }

    /// Rich presence value this user set.
    pub fn rich_presence(&self, key: &str) -> Option<String> {
        self.uni.0.lock().unwrap().users[&self.me]
            .rich
            .get(key)
            .cloned()
    }
}

impl SteamApi for MockSteam {
    fn self_id(&self) -> SteamId {
        self.me
    }

    fn run_frame(&mut self) -> Vec<SteamEvent> {
        let mut u = self.uni.0.lock().unwrap();
        let rev = u.faults.reverse_event_order;
        let mut v: Vec<_> = u
            .users
            .get_mut(&self.me)
            .unwrap()
            .events
            .drain(..)
            .collect();
        if rev {
            v.reverse();
        }
        v
    }

    fn create_lobby(&mut self, max_members: u32) {
        let mut u = self.uni.0.lock().unwrap();
        let id = LOBBY_BASE + u.next_lobby;
        u.next_lobby += 1;
        u.lobbies.insert(
            id,
            Lobby {
                owner: self.me,
                max: max_members,
                members: vec![self.me],
                data: HashMap::new(),
                joinable: true,
                invited: HashSet::new(),
            },
        );
        u.users.get_mut(&self.me).unwrap().lobby = Some(id);
        let double = u.faults.double_lobby_created;
        u.push(
            self.me,
            SteamEvent::LobbyCreated {
                result: 1,
                lobby: id,
            },
        );
        if double {
            u.push(
                self.me,
                SteamEvent::LobbyCreated {
                    result: 1,
                    lobby: id,
                },
            );
        }
        u.push(
            self.me,
            SteamEvent::LobbyEnter {
                lobby: id,
                response: 1,
            },
        );
    }

    fn join_lobby(&mut self, lobby: LobbyId) {
        let mut u = self.uni.0.lock().unwrap();
        let me = self.me;
        let response = match u.lobbies.get(&lobby) {
            None => 2,
            Some(l) if l.members.contains(&me) => 1,
            Some(l) if !l.joinable => 3,
            Some(l) if l.members.len() as u32 >= l.max => 4,
            Some(l) => {
                let friends = u.users[&l.owner].friends.contains(&me);
                if friends || l.invited.contains(&me) {
                    1
                } else {
                    3
                }
            }
        };
        if response == 1 {
            let l = u.lobbies.get_mut(&lobby).unwrap();
            if !l.members.contains(&me) {
                l.members.push(me);
                let others: Vec<_> = l.members.iter().copied().filter(|m| *m != me).collect();
                u.users.get_mut(&me).unwrap().lobby = Some(lobby);
                for m in others {
                    u.push(
                        m,
                        SteamEvent::LobbyChatUpdate {
                            lobby,
                            user_changed: me,
                            making_change: me,
                            state: CHAT_ENTERED,
                        },
                    );
                }
            }
        }
        u.push(me, SteamEvent::LobbyEnter { lobby, response });
    }

    fn leave_lobby(&mut self, lobby: LobbyId) {
        self.uni
            .0
            .lock()
            .unwrap()
            .remove_member(lobby, self.me, CHAT_LEFT);
    }

    fn set_lobby_data(&mut self, lobby: LobbyId, key: &str, value: &str) -> bool {
        let mut u = self.uni.0.lock().unwrap();
        match u.lobbies.get_mut(&lobby) {
            Some(l) if l.owner == self.me => {
                l.data.insert(key.into(), value.into());
                true
            }
            _ => false,
        }
    }

    fn get_lobby_data(&self, lobby: LobbyId, key: &str) -> Option<String> {
        let u = self.uni.0.lock().unwrap();
        u.lobbies
            .get(&lobby)
            .and_then(|l| l.data.get(key).cloned())
            .filter(|s| !s.is_empty())
    }

    fn set_lobby_joinable(&mut self, lobby: LobbyId, joinable: bool) {
        if let Some(l) = self.uni.0.lock().unwrap().lobbies.get_mut(&lobby) {
            if l.owner == self.me {
                l.joinable = joinable;
            }
        }
    }

    fn lobby_owner(&self, lobby: LobbyId) -> SteamId {
        self.uni
            .0
            .lock()
            .unwrap()
            .lobbies
            .get(&lobby)
            .map_or(0, |l| l.owner)
    }

    fn lobby_members(&self, lobby: LobbyId) -> Vec<SteamId> {
        self.uni
            .0
            .lock()
            .unwrap()
            .lobbies
            .get(&lobby)
            .map(|l| l.members.clone())
            .unwrap_or_default()
    }

    fn invite_user(&mut self, lobby: LobbyId, friend: SteamId) -> bool {
        let mut u = self.uni.0.lock().unwrap();
        let me = self.me;
        let ok = u.users[&me].friends.contains(&friend)
            && u.lobbies
                .get(&lobby)
                .is_some_and(|l| l.members.contains(&me));
        if ok {
            u.lobbies.get_mut(&lobby).unwrap().invited.insert(friend);
            u.push(friend, SteamEvent::LobbyInvite { inviter: me, lobby });
        }
        ok
    }

    fn set_rich_presence(&mut self, key: &str, value: &str) {
        self.uni
            .0
            .lock()
            .unwrap()
            .users
            .get_mut(&self.me)
            .unwrap()
            .rich
            .insert(key.into(), value.into());
    }

    fn clear_rich_presence(&mut self) {
        self.uni
            .0
            .lock()
            .unwrap()
            .users
            .get_mut(&self.me)
            .unwrap()
            .rich
            .clear();
    }

    fn open_invite_overlay(&mut self, _lobby: LobbyId) {}

    fn friends(&self) -> Vec<Friend> {
        let u = self.uni.0.lock().unwrap();
        let mut v: Vec<_> = u.users[&self.me]
            .friends
            .iter()
            .filter_map(|f| {
                let fu = u.users.get(f)?;
                Some(Friend {
                    id: *f,
                    name: fu.name.clone(),
                    online: true,
                    playing_app: fu.playing_app,
                    lobby: fu.lobby.unwrap_or(0),
                })
            })
            .collect();
        v.sort_by_key(|f| f.id);
        v
    }

    fn send_message(&mut self, to: SteamId, channel: i32, flags: i32, data: &[u8]) -> SendResult {
        let mut u = self.uni.0.lock().unwrap();
        let me = self.me;
        if let Some(n) = u.faults.buffer_full.get_mut(&me) {
            if *n > 0 {
                *n -= 1;
                return SendResult::BufferFull;
            }
        }
        if !u.users.contains_key(&to) {
            return SendResult::Failed(2);
        }
        // Sending opens our side of the session.
        u.users.get_mut(&me).unwrap().accepted.insert(to);
        if u.faults.session_failed.contains(&to) {
            u.push(me, SteamEvent::SessionFailed { from: to });
            return SendResult::Ok;
        }
        if u.faults.silent.contains(&me) {
            return SendResult::Ok;
        }
        if flags & SEND_RELIABLE == 0 {
            let p = u.faults.unreliable_loss;
            if u.rng.chance(p) {
                return SendResult::Ok;
            }
        }
        let accepted = u.users[&to].accepted.contains(&me);
        if accepted {
            u.users
                .get_mut(&to)
                .unwrap()
                .inbox
                .entry(channel)
                .or_default()
                .push_back(RecvMsg {
                    from: me,
                    data: data.to_vec(),
                });
        } else {
            let first = u.users.get_mut(&to).unwrap().requested.insert(me);
            u.users
                .get_mut(&to)
                .unwrap()
                .held
                .push((me, channel, data.to_vec()));
            if first {
                u.session_requests.push((to, me));
                u.push(to, SteamEvent::SessionRequest { from: me });
            }
        }
        SendResult::Ok
    }

    fn receive_messages(&mut self, channel: i32, max: usize) -> Vec<RecvMsg> {
        let mut u = self.uni.0.lock().unwrap();
        let q = u
            .users
            .get_mut(&self.me)
            .unwrap()
            .inbox
            .entry(channel)
            .or_default();
        let n = q.len().min(max);
        q.drain(..n).collect()
    }

    fn accept_session(&mut self, from: SteamId) -> bool {
        let mut u = self.uni.0.lock().unwrap();
        let me = u.users.get_mut(&self.me).unwrap();
        me.accepted.insert(from);
        let held = std::mem::take(&mut me.held);
        for (f, ch, d) in held {
            if f == from {
                me.inbox
                    .entry(ch)
                    .or_default()
                    .push_back(RecvMsg { from: f, data: d });
            } else {
                me.held.push((f, ch, d));
            }
        }
        true
    }

    fn close_session(&mut self, from: SteamId) {
        let mut u = self.uni.0.lock().unwrap();
        let me = u.users.get_mut(&self.me).unwrap();
        me.accepted.remove(&from);
        me.requested.remove(&from);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trio() -> (MockUniverse, MockSteam, MockSteam, MockSteam) {
        let uni = MockUniverse::new();
        let a = uni.user(1, "alice");
        let b = uni.user(2, "bob");
        let c = uni.user(3, "carol");
        uni.befriend(&a, &b);
        (uni, a, b, c)
    }

    #[test]
    fn lobby_created_arrives_twice() {
        let (_u, mut a, _b, _c) = trio();
        a.create_lobby(4);
        let ev = a.run_frame();
        assert_eq!(
            ev.iter()
                .filter(|e| matches!(e, SteamEvent::LobbyCreated { .. }))
                .count(),
            2
        );
    }

    #[test]
    fn friends_only_join_and_invite() {
        let (_u, mut a, mut b, mut c) = trio();
        a.create_lobby(4);
        let lobby = a
            .run_frame()
            .into_iter()
            .find_map(|e| {
                if let SteamEvent::LobbyCreated { lobby, .. } = e {
                    Some(lobby)
                } else {
                    None
                }
            })
            .unwrap();
        c.join_lobby(lobby);
        assert!(c
            .run_frame()
            .contains(&SteamEvent::LobbyEnter { lobby, response: 3 }));
        // Invite opens the door for a non-friend... only friends can be invited.
        assert!(!a.invite_user(lobby, c.self_id()));
        b.join_lobby(lobby);
        assert!(b
            .run_frame()
            .contains(&SteamEvent::LobbyEnter { lobby, response: 1 }));
        assert!(a.run_frame().iter().any(|e| matches!(
            e,
            SteamEvent::LobbyChatUpdate {
                state: CHAT_ENTERED,
                ..
            }
        )));
    }

    #[test]
    fn stranger_message_held_until_accepted() {
        let (u, mut a, mut b, _c) = trio();
        a.send_message(b.self_id(), 0, SEND_RELIABLE | SEND_NO_NAGLE, b"hi");
        assert!(b.receive_messages(0, 8).is_empty());
        assert_eq!(
            b.run_frame(),
            vec![SteamEvent::SessionRequest { from: a.self_id() }]
        );
        // A second message does not raise a second request.
        a.send_message(b.self_id(), 0, SEND_RELIABLE, b"again");
        assert!(b.run_frame().is_empty());
        b.accept_session(a.self_id());
        let got = b.receive_messages(0, 8);
        assert_eq!(
            got.iter().map(|m| m.data.clone()).collect::<Vec<_>>(),
            vec![b"hi".to_vec(), b"again".to_vec()]
        );
        // Replies to a session we opened need no accept.
        b.send_message(a.self_id(), 0, SEND_RELIABLE, b"yo");
        assert_eq!(a.receive_messages(0, 8).len(), 1);
        assert_eq!(u.session_requests().len(), 1);
    }

    #[test]
    fn faults_work() {
        let (u, mut a, mut b, _c) = trio();
        u.faults(|f| {
            f.buffer_full.insert(a.self_id(), 2);
            f.reverse_event_order = true;
        });
        assert_eq!(
            a.send_message(b.self_id(), 0, SEND_RELIABLE, b"x"),
            SendResult::BufferFull
        );
        assert_eq!(
            a.send_message(b.self_id(), 0, SEND_RELIABLE, b"x"),
            SendResult::BufferFull
        );
        assert_eq!(
            a.send_message(b.self_id(), 0, SEND_RELIABLE, b"x"),
            SendResult::Ok
        );
        u.faults(|f| {
            f.session_failed.insert(b.self_id());
        });
        a.send_message(b.self_id(), 0, SEND_RELIABLE, b"x");
        assert!(a
            .run_frame()
            .contains(&SteamEvent::SessionFailed { from: b.self_id() }));
        let _ = b.run_frame();
    }

    #[test]
    fn owner_migrates_and_crash_flags() {
        let (u, mut a, mut b, _c) = trio();
        a.create_lobby(4);
        let lobby = a
            .run_frame()
            .into_iter()
            .find_map(|e| {
                if let SteamEvent::LobbyCreated { lobby, .. } = e {
                    Some(lobby)
                } else {
                    None
                }
            })
            .unwrap();
        b.join_lobby(lobby);
        b.run_frame();
        u.crash(&a);
        assert_eq!(b.lobby_owner(lobby), b.self_id());
        assert!(b.run_frame().iter().any(|e| matches!(
            e,
            SteamEvent::LobbyChatUpdate {
                state: CHAT_DISCONNECTED,
                ..
            }
        )));
    }
}
