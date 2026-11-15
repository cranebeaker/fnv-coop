//! Rewards the shared story gives (docs/coop/SPECS.md S30). The story's scripts run in the host's
//! game only, and there `Player` is the host's player: a `RewardXP` or a `Player.AddItem` (caps are
//! an item) in a quest's stage or in a line's result gives that player alone. The rules give every
//! player the full reward once (`reward: everyone`, every row of the bundled table): the host's
//! game notes each such reward ([`take`]), and each guest's game gives its own player the same
//! ([`give`]).
//!
//! A reward counts as the story's when the script running is a quest's own or has an owner quest
//! (a stage's result, a quest's script), or is a line's result (its line's quest), unless that
//! quest is one each player plays for themself ([`crate::gate::set_own_quests`]): its rewards are
//! that player's. An object script with no quest runs in every game that has the object, so each
//! game gives its own.
//!
//! A guest's game runs the story's lines an NPC says outside its player's conversation (Sunny
//! handing over her rifle at the bottles): their rewards are noted there too, and the host gives
//! its own player and the other guests the same. Its player's own conversation's results are held
//! for the host and run there.

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;
use std::sync::{Mutex, PoisonError};

use crate::{addr, gate};

/// The wrap at `0094fd10`, `PlayerCharacter::RewardExperience`.
pub(crate) const EXPERIENCE: WrapSlot = WrapSlot::new(Owner::Coop, addr::PLAYER_REWARD_EXPERIENCE);
/// The wrap at `004821a0`, a container's items handed to a reference.
pub(crate) const ITEMS: WrapSlot = WrapSlot::new(Owner::Coop, addr::CONTAINER_GIVE);

/// What a reward is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum What {
    /// Experience points.
    Xp(u32),
    /// Items: the item's form id and how many.
    Item {
        /// The item's form id.
        form: u32,
        /// How many.
        count: u32,
    },
}

/// One reward the story gave in the host's game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Given {
    /// The quest whose script gave it.
    pub quest: u32,
    /// What it was.
    pub what: What,
}

#[derive(Default)]
struct Rewards {
    given: Mutex<Vec<Given>>,
}

/// The story's rewards given in this game since the last call, in order: on the host, for the
/// guests; on a guest, for the host and the other guests.
pub fn take(e: &mut Engine) -> Vec<Given> {
    std::mem::take(
        &mut *e
            .ext_free::<Rewards>()
            .given
            .lock()
            .unwrap_or_else(PoisonError::into_inner),
    )
}

/// Guest: gives this game's player a reward the game's own way, the message included: experience
/// as `RewardXP` does, items as `AddItem` does. False when there is no player yet, or the item is
/// not in this game's data.
pub fn give(e: &mut Engine, what: What) -> bool {
    let player = e.mem.u32(addr::PLAYER_SINGLETON);
    if player == 0 {
        return false;
    }
    match what {
        What::Xp(xp) => {
            e.call(addr::PLAYER_REWARD_EXPERIENCE, &args![player, xp]);
        }
        What::Item { form, count } => {
            let Some(item) = crate::forms::form(e, form) else {
                return false;
            };
            e.with_stack(addr::CONTAINER_SIZE, |e, container| {
                e.call(addr::CONTAINER_CONSTRUCT, &args![container]);
                e.call(addr::CONTAINER_ADD, &args![container, item, count, 0u32]);
                e.call(addr::CONTAINER_GIVE, &args![container, player, 0u32]);
                e.call(addr::CONTAINER_DESTRUCT, &args![container]);
            });
        }
    }
    true
}

/// The story's quest of the script running now, when a reward given now is the story's (see the
/// module); `None` otherwise.
fn story_quest(e: &mut Engine) -> Option<u32> {
    let c = gate::cause(e)?;
    // A guest's game runs no script of the story but a line an NPC says outside the player's
    // conversation (Sunny handing over her rifle at the bottles, PC run 30): its reward is noted
    // too. The story scripts the rules let a guest run itself act on its own player alone.
    if gate::is_guest(e) && c.owner_quest != 0 {
        return None;
    }
    // A line's result script names no owner quest: its line's quest is the one.
    let quest = match c.owner_quest {
        0 => crate::talk::result_quest(e).unwrap_or(0),
        q => q,
    };
    (quest != 0 && !gate::is_own_quest(e, quest)).then_some(quest)
}

fn note(e: &mut Engine, quest: u32, what: What) {
    e.ext_free::<Rewards>()
        .given
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .push(Given { quest, what });
}

/// `PlayerCharacter::RewardExperience` (player, amount): on the host, while a story script runs
/// (see the module), the reward is noted for the guests. It is given here as ever.
pub(crate) fn experience(e: &mut Engine, a: &[u32]) -> Ret {
    let xp = a.get(1).copied().unwrap_or(0);
    if xp != 0 {
        if let Some(quest) = story_quest(e) {
            note(e, quest, What::Xp(xp));
        }
    }
    e.call_next(addr::PLAYER_REWARD_EXPERIENCE, a)
}

/// A container's items handed to a reference (container, reference, quiet): on the host, when
/// the reference is the player and a story script runs (see the module), each item is noted for
/// the guests. They are given here as ever.
pub(crate) fn items(e: &mut Engine, a: &[u32]) -> Ret {
    let (container, target) = (
        a.first().copied().unwrap_or(0),
        a.get(1).copied().unwrap_or(0),
    );
    if container != 0 && target != 0 && target == e.mem.u32(addr::PLAYER_SINGLETON) {
        if let Some(quest) = story_quest(e) {
            let mut node = container.wrapping_add(4);
            let mut seen = 0;
            while node != 0 && seen < 1024 {
                let item = e.mem.u32(node);
                if item != 0 {
                    let count = e.mem.u32(item);
                    let object = e.mem.u32(item.wrapping_add(4));
                    if object != 0 && count as i32 > 0 {
                        let form = e.mem.u32(object.wrapping_add(addr::FORM_ID_OFFSET));
                        note(e, quest, What::Item { form, count });
                    }
                }
                node = e.mem.u32(node.wrapping_add(4));
                seen += 1;
            }
        }
    }
    e.call_next(addr::CONTAINER_GIVE, a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{Story, StorySpec};

    const MQ: u32 = 0x0010_4c0c;
    const OPENING: u32 = 0x0010_4c1c;

    fn story() -> Story {
        let mut s = Story::new(&StorySpec {
            quests: vec![(MQ, vec![10]), (OPENING, vec![10])],
            ..StorySpec::default()
        });
        crate::arm(&mut s.e, true);
        gate::set_own_quests(&mut s.e, [OPENING]);
        s
    }

    /// Harness, 2026-11-13: Trudy's line that ends Sunny's last tutorial (`RewardXP 50`), said
    /// to the guest, ran in the host's game, whose player alone got the 50. On the host, the
    /// story's experience is given here as ever and noted for the guests.
    #[test]
    fn the_storys_experience_on_the_host_is_noted_for_the_guests() {
        let mut s = story();
        s.run_script_rewarding_xp(MQ, 50);
        assert_eq!(s.xp(), 50);
        assert_eq!(
            take(&mut s.e),
            vec![Given {
                quest: MQ,
                what: What::Xp(50)
            }]
        );
        assert_eq!(take(&mut s.e), vec![]);
    }

    /// Not noted: a reward of a quest each player plays for themself, of an object script with no
    /// quest (every game that has the object runs it), one given while the host's state is applied,
    /// and on a guest one of a story script the rules let it run itself. Each is given in its own
    /// game.
    #[test]
    fn other_experience_is_its_own_games() {
        let mut s = story();
        s.run_script_rewarding_xp(OPENING, 10);
        s.run_script_rewarding_xp(0, 20);
        gate::applying(&mut s.e, |e| {
            let player = e.mem.u32(addr::PLAYER_SINGLETON);
            e.call(addr::PLAYER_REWARD_EXPERIENCE, &args![player, 5u32]);
        });
        assert_eq!(s.xp(), 35);
        assert_eq!(take(&mut s.e), vec![]);
        // On a guest, a story script the rules let it run itself (it acts on its own player).
        gate::set_guest(&mut s.e, true);
        gate::set_own_scripts(&mut s.e, [0x0015_0aaa]);
        s.run_named_script_rewarding_xp(0x0015_0aaa, MQ, 40);
        assert_eq!(s.xp(), 75);
        assert_eq!(take(&mut s.e), vec![]);
    }

    /// A guest's game gives its player what the host's noted, the game's own way.
    #[test]
    fn a_guest_gives_its_player_the_experience() {
        let mut s = story();
        gate::set_guest(&mut s.e, true);
        assert!(give(&mut s.e, What::Xp(50)));
        assert_eq!(s.xp(), 50);
        assert_eq!(take(&mut s.e), vec![]);
    }

    const CAPS: u32 = 0x0000_000f;
    const STIMPAK: u32 = 0x0001_5169;

    /// The user, 2026-11-13: caps and items a quest gives must reach both players. On the host,
    /// items the story gives the player (`Player.AddItem`, caps included) are given here as ever
    /// and noted for the guests; items handed to anyone else, or given by a player's own quest
    /// or an object script, are not.
    #[test]
    fn the_storys_items_on_the_host_are_noted_for_the_guests() {
        let mut s = story();
        s.add_item_form(CAPS);
        s.run_script_giving_item(MQ, CAPS, 100);
        assert_eq!(s.items(CAPS), 100);
        assert_eq!(
            take(&mut s.e),
            vec![Given {
                quest: MQ,
                what: What::Item {
                    form: CAPS,
                    count: 100
                }
            }]
        );
        s.run_script_giving_item(OPENING, CAPS, 5);
        s.run_script_giving_item(0, CAPS, 7);
        assert_eq!(s.items(CAPS), 112);
        assert_eq!(take(&mut s.e), vec![]);
        // Two items in one container (a form list's elements): both noted.
        s.add_item_form(STIMPAK);
        s.run_script_giving_two_items(MQ, (CAPS, 1), (STIMPAK, 2));
        assert_eq!(
            take(&mut s.e),
            vec![
                Given {
                    quest: MQ,
                    what: What::Item {
                        form: CAPS,
                        count: 1
                    }
                },
                Given {
                    quest: MQ,
                    what: What::Item {
                        form: STIMPAK,
                        count: 2
                    }
                }
            ]
        );
        // To someone else than the player (a container, an NPC), by a story script.
        s.run_script_giving_item_to(MQ, CAPS, 3, 0x0150_0f00);
        assert_eq!(s.items(CAPS), 113);
        assert_eq!(take(&mut s.e), vec![]);
    }

    /// A guest's game gives its player items the host noted, as `AddItem` does; an item its data
    /// lacks is not given.
    #[test]
    fn a_guest_gives_its_player_the_items() {
        let mut s = story();
        s.add_item_form(CAPS);
        gate::set_guest(&mut s.e, true);
        assert!(give(
            &mut s.e,
            What::Item {
                form: CAPS,
                count: 100
            }
        ));
        assert_eq!(s.items(CAPS), 100);
        assert!(!give(
            &mut s.e,
            What::Item {
                form: 0x00ab_cdef,
                count: 1
            }
        ));
        assert_eq!(take(&mut s.e), vec![]);
    }
}
