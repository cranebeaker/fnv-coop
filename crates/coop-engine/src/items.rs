//! A reference's inventory, item by item (docs/coop/SPECS.md S18): what a container, a body, a
//! trader or the player holds, read and changed through the game's own functions.
//!
//! - **Reading.** The items are the base container's objects and the entries of the reference's
//!   inventory changes, walked as the built-in script extender walks them (`invref.rs` of
//!   `nvse`, `inventory_items`); each one's count is the game's own
//!   (`InventoryChanges::GetItemCount`). A stack that has a health of its own is listed with its
//!   condition, as a part of the item's full health (what the game's temporary container takes
//!   when an item is added).
//! - **Adding** is what the `AddItem` command does: a temporary container with the item, given
//!   to the reference. **Removing** is what `RemoveItem` does: the reference's own removal
//!   (virtual slot 0x17c).
//! - **Caps** are the item with form id 0xf, read with the game's gold count.
//! - **What the player is about to move.** Co-op wraps `ContainerMenu::Create`,
//!   `BarterMenu::Create` and the player's pick-up as [`Owner::Coop`]. While a session shares
//!   items ([`set_sharing`]) they note which reference's menu opened ([`take_opened`]) and which
//!   placed item the player took ([`take_picked`]); every call goes on to the game unchanged.
//!   With sharing off they note nothing.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::{actors, addr, forms, live};

/// Co-op's places in front of the two menus' `Create` and the player's pick-up.
pub(crate) const CONTAINER_CREATE: WrapSlot =
    WrapSlot::new(Owner::Coop, addr::CONTAINER_MENU_CREATE);
/// See [`CONTAINER_CREATE`].
pub(crate) const BARTER_CREATE: WrapSlot = WrapSlot::new(Owner::Coop, addr::BARTER_MENU_CREATE);
/// See [`CONTAINER_CREATE`].
pub(crate) const PICK_UP: WrapSlot = WrapSlot::new(Owner::Coop, addr::PLAYER_PICK_UP);

/// The most nodes of one of the game's lists a walk goes through (a list that runs back into
/// itself ends there).
const MAX_NODES: usize = 1 << 16;

/// One stack of an item in an inventory.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stack {
    /// The item's form id.
    pub item: u32,
    /// How many.
    pub count: i32,
    /// The stack's condition, from 0 to 1 of the item's full health, where the game keeps one
    /// for it; `None` for a stack in the item's own state.
    pub condition: Option<f32>,
}

/// A placed item the player took from the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Picked {
    /// The placed reference's form id.
    pub reference: u32,
    /// The item's form id.
    pub item: u32,
    /// How many the player got.
    pub count: i32,
}

/// The game-wide state (a lock-free shared slot, as the journal's).
#[derive(Debug, Default)]
pub struct Items {
    sharing: AtomicBool,
    opened: Mutex<Vec<u32>>,
    picked: Mutex<Vec<Picked>>,
    missing: Mutex<Vec<u32>>,
}

fn locked<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Starts or stops noting what the player opens and picks up: with it off the wraps change and
/// note nothing.
pub fn set_sharing(e: &mut Engine, on: bool) {
    e.ext_free::<Items>().sharing.store(on, Ordering::SeqCst);
}

fn sharing(e: &mut Engine) -> bool {
    e.ext_free::<Items>().sharing.load(Ordering::SeqCst)
}

/// The form ids of the references whose container or barter menu opened since the last call (a
/// trader and, when it has one, its merchant container).
pub fn take_opened(e: &mut Engine) -> Vec<u32> {
    std::mem::take(&mut *locked(&e.ext_free::<Items>().opened))
}

/// The placed items the player took from the world since the last call.
pub fn take_picked(e: &mut Engine) -> Vec<Picked> {
    std::mem::take(&mut *locked(&e.ext_free::<Items>().picked))
}

/// The addresses [`install`] found nothing at, so that part of co-op is off in this game.
pub fn missing(e: &mut Engine) -> Vec<u32> {
    locked(&e.ext_free::<Items>().missing).clone()
}

/// The wraps; called by [`crate::install`] before the game's threads start. An address with
/// nothing registered is left out and named by [`missing`].
pub(crate) fn install(e: &mut Engine) {
    let wraps: [(u32, WrapSlot, AbiFn); 3] = [
        (
            addr::CONTAINER_MENU_CREATE,
            CONTAINER_CREATE,
            container_create,
        ),
        (addr::BARTER_MENU_CREATE, BARTER_CREATE, barter_create),
        (addr::PLAYER_PICK_UP, PICK_UP, pick_up),
    ];
    let mut missing = Vec::new();
    for (at, slot, f) in wraps {
        if e.is_registered(at) {
            e.wrap(slot, f);
        } else {
            missing.push(at);
        }
    }
    *locked(&e.ext_free::<Items>().missing) = missing;
}

fn id_of(e: &mut Engine, form: u32) -> u32 {
    if form == 0 {
        0
    } else {
        e.mem.u32(form.wrapping_add(addr::FORM_ID_OFFSET))
    }
}

fn note_opened(e: &mut Engine, reference: u32) {
    let id = id_of(e, reference);
    if id != 0 {
        locked(&e.ext_free::<Items>().opened).push(id);
    }
}

/// The wrap at `0075b310`. Its words: the reference, the mode.
pub(crate) fn container_create(e: &mut Engine, a: &[u32]) -> Ret {
    let reference = a.first().copied().unwrap_or(0);
    if sharing(e) && reference != 0 {
        note_opened(e, reference);
    }
    e.call_next(addr::CONTAINER_MENU_CREATE, a)
}

/// The wrap at `0072d250`. Its words: the vendor, the discount.
pub(crate) fn barter_create(e: &mut Engine, a: &[u32]) -> Ret {
    let vendor = a.first().copied().unwrap_or(0);
    if sharing(e) && vendor != 0 {
        note_opened(e, vendor);
        let stock = e
            .call(
                addr::GET_MERCHANT_CONTAINER,
                &args![vendor.wrapping_add(addr::REF_EXTRA_LIST_OFFSET)],
            )
            .u32();
        if stock != 0 {
            note_opened(e, stock);
        }
    }
    e.call_next(addr::BARTER_MENU_CREATE, a)
}

/// The wrap at `00953ff0`. Its words: the player, the placed reference, the count, a byte. What
/// the player got is counted around the game's own pick-up.
pub(crate) fn pick_up(e: &mut Engine, a: &[u32]) -> Ret {
    let (player, reference) = (
        a.first().copied().unwrap_or(0),
        a.get(1).copied().unwrap_or(0),
    );
    if !sharing(e) || player == 0 || reference == 0 {
        return e.call_next(addr::PLAYER_PICK_UP, a);
    }
    let id = id_of(e, reference);
    let base = e
        .mem
        .u32(reference.wrapping_add(addr::REF_BASE_FORM_OFFSET));
    let item = id_of(e, base);
    let before = if base == 0 {
        0
    } else {
        count_form(e, player, base)
    };
    let r = e.call_next(addr::PLAYER_PICK_UP, a);
    if base != 0 {
        let got = count_form(e, player, base) - before;
        if got > 0 && id != 0 && item != 0 {
            locked(&e.ext_free::<Items>().picked).push(Picked {
                reference: id,
                item,
                count: got,
            });
        }
    }
    r
}

/// The reference with id `id` that can hold items in this game now: a placed object or a placed
/// actor (the player among them).
pub fn holder(e: &mut Engine, id: u32) -> Option<u32> {
    let f = forms::form(e, id)?;
    matches!(
        e.mem.u8(f.wrapping_add(addr::FORM_TYPE_OFFSET)),
        addr::FORM_TYPE_REFR | addr::FORM_TYPE_ACHR | addr::FORM_TYPE_ACRE
    )
    .then_some(f)
}

/// Whether the two games can mean the same holder by `id`: a reference of the plugins, or the
/// player's own (each game's player is its own, so it is never shared).
pub fn shareable(e: &mut Engine, id: u32) -> bool {
    id != 0 && id < actors::FIRST_DYNAMIC_ID && live::player(e).map(|p| id_of(e, p)) != Some(id)
}

/// The item form with id `id`: a form the game's own item commands would add (virtual slot
/// 0xe4).
pub fn item_form(e: &mut Engine, id: u32) -> Option<u32> {
    let f = forms::form(e, id)?;
    e.vcall(f, addr::FORM_IS_ITEM_SLOT, &args![])
        .bool()
        .then_some(f)
}

/// The data of each node of one of the game's lists (data, next), empty nodes left out.
fn list_items(e: &mut Engine, list: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let mut node = list;
    while node != 0 && out.len() < MAX_NODES {
        let data = e.mem.u32(node);
        if data != 0 {
            out.push(data);
        }
        node = e.mem.u32(node.wrapping_add(4));
    }
    out
}

/// The reference's base container, or 0 (`nvse`'s `container_of`).
fn base_container(e: &mut Engine, r: u32) -> u32 {
    let base = e.mem.u32(r.wrapping_add(addr::REF_BASE_FORM_OFFSET));
    if base == 0 {
        return 0;
    }
    if e.vcall(r, addr::FORM_IS_ACTOR_SLOT, &args![]).bool() {
        return base + addr::ACTOR_BASE_CONTAINER_OFFSET;
    }
    if e.mem.u8(base + addr::FORM_TYPE_OFFSET) == addr::FORM_TYPE_CONTAINER {
        return base + addr::CONT_CONTAINER_OFFSET;
    }
    0
}

fn changes(e: &mut Engine, r: u32) -> u32 {
    e.call(addr::REF_INVENTORY_CHANGES, &args![r]).u32()
}

/// The game's count of item form `form` in reference `r`.
fn count_form(e: &mut Engine, r: u32, form: u32) -> i32 {
    match changes(e, r) {
        0 => 0,
        c => e.call(addr::CHANGES_ITEM_COUNT, &args![c, form]).i32(),
    }
}

/// The full health of item form `form`, or 0 for an item without health.
fn full_health(e: &mut Engine, form: u32) -> u32 {
    match e.call(addr::FORM_AS_HEALTH_FORM, &args![form]).u32() {
        0 => 0,
        h => e.mem.u32(h + addr::HEALTH_FORM_HEALTH_OFFSET),
    }
}

/// `(count, health)` of a stack's extra data list: its count (1 without one) and its own health.
fn stack_of(e: &mut Engine, list: u32) -> (i32, Option<f32>) {
    let count = match e
        .call(
            addr::EXTRA_GET_BY_TYPE,
            &args![list, addr::EXTRA_TYPE_COUNT],
        )
        .u32()
    {
        0 => 1,
        x => i32::from(e.mem.u16(x + addr::EXTRA_VALUE_OFFSET) as i16),
    };
    let health = match e
        .call(
            addr::EXTRA_GET_BY_TYPE,
            &args![list, addr::EXTRA_TYPE_HEALTH],
        )
        .u32()
    {
        0 => None,
        x => Some(e.mem.f32(x + addr::EXTRA_VALUE_OFFSET)),
    };
    (count, health)
}

/// Everything reference `r` holds, stack by stack, in the order the game lists it: the base
/// container's items first, then the items only its changes hold. An item's stacks with a health
/// of their own come first, then the rest of its count.
pub fn list(e: &mut Engine, r: u32) -> Vec<Stack> {
    let inv = changes(e, r);
    if inv == 0 {
        return Vec::new();
    }
    let entries = list_items(e, e.mem.u32(inv));
    let mut forms: Vec<u32> = Vec::new();
    let container = base_container(e, r);
    if container != 0 {
        let head = container + addr::CONTAINER_LIST_OFFSET;
        for object in list_items(e, head) {
            let form = e.mem.u32(object + addr::CONTAINER_OBJECT_FORM_OFFSET);
            if form != 0
                && e.mem.u8(form + addr::FORM_TYPE_OFFSET) != addr::FORM_TYPE_LEVELED_ITEM
                && !forms.contains(&form)
            {
                forms.push(form);
            }
        }
    }
    for entry in &entries {
        let form = e.mem.u32(entry + addr::ENTRY_FORM_OFFSET);
        if form != 0 && !forms.contains(&form) {
            forms.push(form);
        }
    }
    let mut out = Vec::new();
    for form in forms {
        let mut left = e.call(addr::CHANGES_ITEM_COUNT, &args![inv, form]).i32();
        let item = id_of(e, form);
        if left <= 0 || item == 0 {
            continue;
        }
        let full = full_health(e, form);
        let entry = entries
            .iter()
            .copied()
            .find(|en| e.mem.u32(en + addr::ENTRY_FORM_OFFSET) == form);
        if let (Some(entry), true) = (entry, full != 0) {
            let extend = e.mem.u32(entry + addr::ENTRY_EXTEND_OFFSET);
            for xdata in list_items(e, extend) {
                let (n, health) = stack_of(e, xdata);
                let Some(health) = health else {
                    continue;
                };
                let n = n.min(left);
                if n <= 0 {
                    continue;
                }
                left -= n;
                out.push(Stack {
                    item,
                    count: n,
                    condition: Some((health / full as f32).clamp(0.0, 1.0)),
                });
            }
        }
        if left > 0 {
            out.push(Stack {
                item,
                count: left,
                condition: None,
            });
        }
    }
    out
}

/// What reference `r` holds as a count per item form id.
pub fn totals(e: &mut Engine, r: u32) -> BTreeMap<u32, i32> {
    let mut out = BTreeMap::new();
    for s in list(e, r) {
        *out.entry(s.item).or_insert(0) += s.count;
    }
    out
}

/// How many of item `item` (a form id) reference `r` holds; 0 for a form that is not in the game.
pub fn count(e: &mut Engine, r: u32, item: u32) -> i32 {
    match forms::form(e, item) {
        Some(form) => count_form(e, r, form).max(0),
        None => 0,
    }
}

/// Gives reference `r` `count` of item `item`, as the `AddItem` command does and without the
/// player's message; with `condition` every one of them at that part of the item's full health.
/// False when the form is not an item of this game or the count is not positive.
pub fn add(e: &mut Engine, r: u32, item: u32, count: i32, condition: Option<f32>) -> bool {
    if count <= 0 {
        return false;
    }
    let Some(form) = item_form(e, item) else {
        return false;
    };
    e.with_stack(addr::TEMP_CONTAINER_SIZE, |e, container| {
        let c = container.addr();
        e.call(addr::TEMP_CONTAINER_CONSTRUCT, &args![c]);
        e.call(
            addr::TEMP_CONTAINER_ADD,
            &args![c, form, count as u32, 0u32],
        );
        if let Some(part) = condition {
            let bits = part.clamp(0.0, 1.0).to_bits();
            e.call(addr::TEMP_CONTAINER_SET_HEALTH, &args![c, bits]);
        }
        e.call(addr::TEMP_CONTAINER_GIVE, &args![c, r, 1u32]);
        e.call(addr::TEMP_CONTAINER_DESTRUCT, &args![c]);
    });
    true
}

/// Takes up to `count` of item `item` out of reference `r`, as the `RemoveItem` command does for
/// a reference (the game chooses the stacks). Answers how many went.
pub fn remove(e: &mut Engine, r: u32, item: u32, count: i32) -> i32 {
    let Some(form) = forms::form(e, item) else {
        return 0;
    };
    let n = count.min(count_form(e, r, form));
    if n <= 0 {
        return 0;
    }
    e.vcall(
        r,
        addr::REF_REMOVE_ITEM_SLOT,
        &args![form, 0u32, n as u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
    n
}

/// The caps reference `r` has (the game's own gold count).
pub fn caps(e: &mut Engine, r: u32) -> i32 {
    match changes(e, r) {
        0 => 0,
        c => e.call(addr::CHANGES_GOLD_AMOUNT, &args![c]).i32(),
    }
}

/// Makes reference `r`'s caps `to` by adding or removing the caps item. False when the game has
/// no caps item.
pub fn set_caps(e: &mut Engine, r: u32, to: i32) -> bool {
    if forms::form(e, addr::CAPS_FORM_ID).is_none() {
        return false;
    }
    let now = caps(e, r);
    let to = to.max(0);
    if to > now {
        add(e, r, addr::CAPS_FORM_ID, to - now, None);
    } else if to < now {
        remove(e, r, addr::CAPS_FORM_ID, now - to);
    }
    true
}

/// Whether placed reference `id` is still to be had in this game: in the game and not disabled.
pub fn in_world(e: &mut Engine, id: u32) -> bool {
    forms::form(e, id).is_some_and(|f| {
        e.mem.u8(f + addr::FORM_TYPE_OFFSET) == addr::FORM_TYPE_REFR
            && e.mem.u32(f + addr::FORM_FLAGS_OFFSET) & addr::FORM_FLAG_DISABLED == 0
    })
}

/// Takes placed reference `id` out of this game's world (`TESObjectREFR::Disable`), as when
/// another player picked it up. False when it was not to be had.
pub fn take_from_world(e: &mut Engine, id: u32) -> bool {
    if !in_world(e, id) {
        return false;
    }
    if let Some(f) = forms::form(e, id) {
        e.call(addr::REF_DISABLE, &args![f]);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::itemkit::{self as kit, HolderSpec, ItemKitSpec, ItemSpec, Kind, WorldItem};
    use crate::testkit::{Story, StorySpec};

    const CELL: u32 = 0x0010_5a00;
    const CHEST: u32 = 0x0010_5b01;
    const TRADER: u32 = 0x0010_5b02;
    const STOCK: u32 = 0x0010_5b03;
    const BOTTLE: u32 = 0x0010_5b04;
    const CAPS: u32 = addr::CAPS_FORM_ID;
    const STIMPAK: u32 = 0x0001_5169;
    const RIFLE: u32 = 0x0000_4333;

    fn story() -> Story {
        let mut s = Story::new(&StorySpec {
            cells: vec![CELL],
            ..StorySpec::default()
        });
        kit::install(
            &mut s,
            &ItemKitSpec {
                items: vec![
                    ItemSpec {
                        id: STIMPAK,
                        health: 0,
                    },
                    ItemSpec {
                        id: RIFLE,
                        health: 200,
                    },
                ],
                holders: vec![
                    HolderSpec {
                        id: CHEST,
                        kind: Kind::Container,
                        base: vec![(STIMPAK, 2)],
                        items: vec![(CAPS, 30, None), (RIFLE, 1, Some(0.5))],
                        merchant: 0,
                    },
                    HolderSpec {
                        id: TRADER,
                        kind: Kind::Actor,
                        base: vec![],
                        items: vec![],
                        merchant: STOCK,
                    },
                    HolderSpec {
                        id: STOCK,
                        kind: Kind::Container,
                        base: vec![],
                        items: vec![(CAPS, 100, None)],
                        merchant: 0,
                    },
                ],
                world: vec![WorldItem {
                    id: BOTTLE,
                    item: STIMPAK,
                    count: 1,
                }],
            },
        );
        s
    }

    #[test]
    fn a_holder_is_listed_with_base_items_changes_and_conditions() {
        let mut s = story();
        let chest = holder(&mut s.e, CHEST).unwrap();
        assert_eq!(
            list(&mut s.e, chest),
            [
                Stack {
                    item: STIMPAK,
                    count: 2,
                    condition: None
                },
                Stack {
                    item: CAPS,
                    count: 30,
                    condition: None
                },
                Stack {
                    item: RIFLE,
                    count: 1,
                    condition: Some(0.5)
                },
            ]
        );
        assert_eq!(count(&mut s.e, chest, STIMPAK), 2);
        assert_eq!(count(&mut s.e, chest, 0x0099_9999), 0);
        assert_eq!(caps(&mut s.e, chest), 30);
        // A form of another kind is not a holder; an unknown id is none.
        assert_eq!(holder(&mut s.e, STIMPAK), None);
        assert_eq!(holder(&mut s.e, 0x0099_9999), None);
    }

    #[test]
    fn items_are_added_and_removed_through_the_games_functions() {
        let mut s = story();
        let chest = holder(&mut s.e, CHEST).unwrap();
        // Taking one of the base container's two leaves one (an entry of -1 beside the base).
        assert_eq!(remove(&mut s.e, chest, STIMPAK, 1), 1);
        assert_eq!(count(&mut s.e, chest, STIMPAK), 1);
        // No more go than are there.
        assert_eq!(remove(&mut s.e, chest, STIMPAK, 5), 1);
        assert_eq!(remove(&mut s.e, chest, STIMPAK, 1), 0);
        assert_eq!(totals(&mut s.e, chest).get(&STIMPAK), None);
        // An item added with a condition is a stack with that health; one without is plain.
        assert!(add(&mut s.e, chest, RIFLE, 2, Some(0.25)));
        assert!(add(&mut s.e, chest, STIMPAK, 3, None));
        let l = list(&mut s.e, chest);
        assert!(l.contains(&Stack {
            item: RIFLE,
            count: 2,
            condition: Some(0.25)
        }));
        assert!(l.contains(&Stack {
            item: STIMPAK,
            count: 3,
            condition: None
        }));
        assert_eq!(totals(&mut s.e, chest)[&RIFLE], 3);
        // Not an item, not a positive count: nothing is given.
        assert!(!add(&mut s.e, chest, CHEST, 1, None));
        assert!(!add(&mut s.e, chest, STIMPAK, 0, None));
        assert_eq!(count(&mut s.e, chest, STIMPAK), 3);
        // Every change went through the journal as an inventory change of the holder.
        assert_eq!(kit::changes_seen(&mut s), 4);
    }

    #[test]
    fn caps_are_read_and_set_on_a_holder_and_the_player() {
        let mut s = story();
        let chest = holder(&mut s.e, CHEST).unwrap();
        let player = live::player(&mut s.e).unwrap();
        assert!(set_caps(&mut s.e, chest, 12));
        assert_eq!(caps(&mut s.e, chest), 12);
        assert_eq!(caps(&mut s.e, player), 0);
        assert!(set_caps(&mut s.e, player, 250));
        assert_eq!(caps(&mut s.e, player), 250);
        assert!(set_caps(&mut s.e, player, 40));
        assert_eq!(count(&mut s.e, player, CAPS), 40);
        assert!(!shareable(&mut s.e, 0x14));
        assert!(shareable(&mut s.e, CHEST));
        assert!(!shareable(&mut s.e, 0xff00_0001));
    }

    #[test]
    fn the_wraps_note_nothing_with_sharing_off_and_pass_every_call_on() {
        let mut s = story();
        kit::open_container(&mut s, CHEST);
        kit::open_barter(&mut s, TRADER);
        assert!(kit::pick_up(&mut s, BOTTLE));
        assert!(take_opened(&mut s.e).is_empty());
        assert!(take_picked(&mut s.e).is_empty());
        // The game's own functions ran all the same.
        assert_eq!(kit::menus_opened(&mut s), 2);
        let player = live::player(&mut s.e).unwrap();
        assert_eq!(count(&mut s.e, player, STIMPAK), 1);
        assert!(!in_world(&mut s.e, BOTTLE));
    }

    #[test]
    fn with_sharing_on_the_wraps_note_the_opened_holders_and_the_pick_up() {
        let mut s = story();
        assert!(missing(&mut s.e).is_empty());
        for at in [
            addr::CONTAINER_MENU_CREATE,
            addr::BARTER_MENU_CREATE,
            addr::PLAYER_PICK_UP,
        ] {
            assert_eq!(s.e.wraps_at(at), [Owner::Coop], "{at:08x}");
        }
        assert!(s.e.unlisted_wraps().is_empty());
        set_sharing(&mut s.e, true);
        kit::open_container(&mut s, CHEST);
        // A trader's menu names the trader and its merchant container.
        kit::open_barter(&mut s, TRADER);
        assert_eq!(take_opened(&mut s.e), [CHEST, TRADER, STOCK]);
        assert!(take_opened(&mut s.e).is_empty());
        assert_eq!(kit::menus_opened(&mut s), 2);
        assert!(in_world(&mut s.e, BOTTLE));
        assert!(kit::pick_up(&mut s, BOTTLE));
        assert_eq!(
            take_picked(&mut s.e),
            [Picked {
                reference: BOTTLE,
                item: STIMPAK,
                count: 1
            }]
        );
        // A second pick-up of what is gone gives nothing and notes nothing.
        assert!(!kit::pick_up(&mut s, BOTTLE));
        assert!(take_picked(&mut s.e).is_empty());
        // Another player's take: the reference leaves this world once.
        let mut other = story();
        assert!(take_from_world(&mut other.e, BOTTLE));
        assert!(!take_from_world(&mut other.e, BOTTLE));
    }
}
