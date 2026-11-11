//! Stand-ins for the game's inventories, for tests of [`crate::items`] and of the session's loot
//! and barter (docs/coop/SPECS.md S18): holders (containers, actors, the story's player) with a
//! base container and inventory changes laid out in engine memory as the game lays them out,
//! item forms, placed items, and the functions co-op calls on them. They do only what co-op
//! relies on; nothing here says how the real functions behave beyond that.

use std::collections::BTreeMap;

use engine_core::prelude::*;

use crate::testkit::{self, Story};
use crate::{addr, forms, items, live};

/// The kit's memory: forms, lists and extra data, handed out upwards.
const IK: u32 = 0x0500_0000;
const IK_LEN: u32 = 0x0040_0000;
/// The vtables of the kit's references, actors and items.
const REF_VT: u32 = IK;
const ACTOR_VT: u32 = IK + 0x400;
const ITEM_VT: u32 = IK + 0x800;
const FIRST: u32 = IK + 0x1000;
/// The kit's functions behind virtual slots.
const FNS: u32 = 0x0132_0000;
const TRUE_FN: u32 = FNS;
const FALSE_FN: u32 = FNS + 0x10;
const REMOVE_FN: u32 = FNS + 0x20;
/// A kit form's size; a placed item keeps its count at +0x70, an item form its health part at
/// +0x40.
const FORM_LEN: u32 = 0x80;
const WORLD_COUNT: u32 = 0x70;
const HEALTH_PART: u32 = 0x40;
/// Where a stack's extra data list keeps its count and health extras in the kit.
const XDATA_COUNT: u32 = 0x10;
const XDATA_HEALTH: u32 = 0x14;
/// The form type of the kit's items and of an actor's base (any that is no container).
const FORM_TYPE_ITEM: u8 = 0x1f;
const FORM_TYPE_NPC: u8 = 0x2a;

/// What kind of holder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Kind {
    /// A placed container.
    #[default]
    Container,
    /// A placed actor (a trader, a body).
    Actor,
}

/// An item form.
#[derive(Debug, Clone, Copy, Default)]
pub struct ItemSpec {
    /// Its form id.
    pub id: u32,
    /// Its full health (0: an item without health).
    pub health: u32,
}

/// A holder.
#[derive(Debug, Clone, Default)]
pub struct HolderSpec {
    /// Its reference id.
    pub id: u32,
    /// Container or actor.
    pub kind: Kind,
    /// Its base container: (item form id, count).
    pub base: Vec<(u32, i32)>,
    /// What it holds beside the base container: (item form id, count, condition).
    pub items: Vec<(u32, i32, Option<f32>)>,
    /// The reference id of its merchant container (0: none).
    pub merchant: u32,
}

/// An item lying in the world.
#[derive(Debug, Clone, Copy, Default)]
pub struct WorldItem {
    /// The placed reference's id.
    pub id: u32,
    /// The item's form id.
    pub item: u32,
    /// How many.
    pub count: i32,
}

/// What the kit adds to a story. The caps item (form id 0xf) is always there, and the story's
/// player (when it has one) is a holder with nothing.
#[derive(Debug, Clone, Default)]
pub struct ItemKitSpec {
    /// Item forms.
    pub items: Vec<ItemSpec>,
    /// Holders.
    pub holders: Vec<HolderSpec>,
    /// Placed items.
    pub world: Vec<WorldItem>,
}

#[derive(Debug, Default)]
struct Holder {
    /// The base container's objects: (item form, count).
    base: Vec<(u32, i32)>,
    /// Everything held now: (item form, count, condition).
    have: Vec<(u32, i32, Option<f32>)>,
    inv: u32,
    container: u32,
    merchant: u32,
}

/// The kit's state beside the engine's memory.
#[derive(Debug, Default)]
struct Kit {
    next: u32,
    holders: BTreeMap<u32, Holder>,
    temps: BTreeMap<u32, Vec<(u32, i32, Option<f32>)>>,
    changes: u32,
    menus: u32,
}

fn eax(v: u32) -> Ret {
    Ret {
        eax: v,
        ..Ret::default()
    }
}

fn new(e: &mut Engine, size: u32) -> u32 {
    let k = e.ext::<Kit>();
    let at = k.next;
    k.next += (size + 7) & !7;
    assert!(k.next <= IK + IK_LEN, "the item kit's memory is used up");
    at
}

fn add_change(e: &mut Engine, form: u32, flags: u32) {
    e.ext::<Kit>().changes += 1;
    e.call(addr::SAVE_LOAD_ADD_CHANGE, &args![0u32, form, flags, 0u32]);
}

/// Writes a list (data, next) of `items` and answers its first node (an empty node for none).
fn write_list(e: &mut Engine, items: &[u32]) -> u32 {
    let head = new(e, 8 * items.len().max(1) as u32);
    e.mem.set_u32(head, 0);
    e.mem.set_u32(head + 4, 0);
    for (i, it) in items.iter().enumerate() {
        let node = head + 8 * i as u32;
        e.mem.set_u32(node, *it);
        let next = if i + 1 < items.len() { node + 8 } else { 0 };
        e.mem.set_u32(node + 4, next);
    }
    head
}

/// Lays holder `r`'s base container and inventory changes out anew from the kit's state.
fn rebuild(e: &mut Engine, r: u32) {
    let (base, have, inv, container) = {
        let h = &e.ext::<Kit>().holders[&r];
        (h.base.clone(), h.have.clone(), h.inv, h.container)
    };
    let mut objects = Vec::new();
    for (form, count) in &base {
        let o = new(e, 8);
        e.mem
            .set_u32(o + addr::CONTAINER_OBJECT_COUNT_OFFSET, *count as u32);
        e.mem.set_u32(o + addr::CONTAINER_OBJECT_FORM_OFFSET, *form);
        objects.push(o);
    }
    let list = write_list(e, &objects);
    e.mem
        .set_u32(container + addr::CONTAINER_LIST_OFFSET, e.mem.u32(list));
    e.mem.set_u32(
        container + addr::CONTAINER_LIST_OFFSET + 4,
        e.mem.u32(list + 4),
    );
    let mut forms: Vec<u32> = base.iter().map(|b| b.0).collect();
    for (form, _, _) in &have {
        if !forms.contains(form) {
            forms.push(*form);
        }
    }
    let mut entries = Vec::new();
    for form in forms {
        let total: i32 = have.iter().filter(|h| h.0 == form).map(|h| h.1).sum();
        let in_base: i32 = base.iter().filter(|b| b.0 == form).map(|b| b.1).sum();
        let full = e
            .mem
            .u32(form + HEALTH_PART + addr::HEALTH_FORM_HEALTH_OFFSET);
        let mut stacks = Vec::new();
        for (_, count, condition) in have.iter().filter(|h| h.0 == form) {
            let Some(part) = condition else {
                continue;
            };
            let x = new(e, 0x20);
            let c = new(e, 0x10);
            e.mem.set_u16(c + addr::EXTRA_VALUE_OFFSET, *count as u16);
            let h = new(e, 0x10);
            e.mem
                .set_f32(h + addr::EXTRA_VALUE_OFFSET, part * full as f32);
            e.mem
                .set_u32(x + XDATA_COUNT, if *count == 1 { 0 } else { c });
            e.mem.set_u32(x + XDATA_HEALTH, h);
            stacks.push(x);
        }
        if total == in_base && stacks.is_empty() {
            continue;
        }
        let entry = new(e, 0xc);
        let extend = if stacks.is_empty() {
            0
        } else {
            write_list(e, &stacks)
        };
        e.mem.set_u32(entry + addr::ENTRY_EXTEND_OFFSET, extend);
        e.mem
            .set_u32(entry + addr::ENTRY_COUNT_OFFSET, (total - in_base) as u32);
        e.mem.set_u32(entry + addr::ENTRY_FORM_OFFSET, form);
        entries.push(entry);
    }
    let head = write_list(e, &entries);
    e.mem.set_u32(inv, head);
}

fn give(e: &mut Engine, r: u32, form: u32, count: i32, condition: Option<f32>) {
    let Some(h) = e.ext::<Kit>().holders.get_mut(&r) else {
        return;
    };
    match h
        .have
        .iter_mut()
        .find(|s| s.0 == form && s.2 == condition && condition.is_none())
    {
        Some(s) => s.1 += count,
        None => h.have.push((form, count, condition)),
    }
    rebuild(e, r);
    add_change(e, r, addr::CHANGE_REFR_INVENTORY);
}

/// Takes up to `count` of `form` out of `r`: the stacks that went, the plain ones first.
fn take(e: &mut Engine, r: u32, form: u32, count: i32) -> Vec<(i32, Option<f32>)> {
    let Some(h) = e.ext::<Kit>().holders.get_mut(&r) else {
        return Vec::new();
    };
    let mut left = count;
    let mut out = Vec::new();
    for plain in [true, false] {
        for s in h
            .have
            .iter_mut()
            .filter(|s| s.0 == form && s.2.is_none() == plain)
        {
            let n = s.1.min(left);
            if n > 0 {
                s.1 -= n;
                left -= n;
                out.push((n, s.2));
            }
        }
    }
    h.have.retain(|s| s.1 > 0);
    if !out.is_empty() {
        rebuild(e, r);
        add_change(e, r, addr::CHANGE_REFR_INVENTORY);
    }
    out
}

/// Virtual slot 0x17c: `(this, item, extra, count, 0, 0, target, 0, 0, 1, 0)`.
fn remove_fn(e: &mut Engine, a: &[u32]) -> Ret {
    let (r, form, count, target) = (a[0], a[1], a[3] as i32, a[6]);
    for (n, condition) in take(e, r, form, count) {
        if target != 0 {
            give(e, target, form, n, condition);
        }
    }
    Ret::default()
}

fn holder_of_inv(e: &mut Engine, inv: u32) -> Option<u32> {
    e.ext::<Kit>()
        .holders
        .iter()
        .find(|(_, h)| h.inv == inv)
        .map(|(r, _)| *r)
}

fn held(e: &mut Engine, inv: u32, pick: impl Fn(&Engine, u32) -> bool) -> i32 {
    let Some(r) = holder_of_inv(e, inv) else {
        return 0;
    };
    let have = e.ext::<Kit>().holders[&r].have.clone();
    have.iter().filter(|s| pick(e, s.0)).map(|s| s.1).sum()
}

fn make_form(e: &mut Engine, vtable: u32, kind: u8, id: u32) -> u32 {
    let f = new(e, FORM_LEN);
    e.mem.set_u32(f, vtable);
    e.mem.set_u8(f + addr::FORM_TYPE_OFFSET, kind);
    e.mem.set_u32(f + addr::FORM_ID_OFFSET, id);
    if id != 0 {
        testkit::add_form(e, id, f);
    }
    f
}

fn item(e: &mut Engine, id: u32) -> u32 {
    forms::form(e, id).expect("an item of the kit")
}

/// Makes `r` a holder: a base form of its kind with the base container, and its changes.
fn make_holder(e: &mut Engine, r: u32, spec: &HolderSpec) {
    let base_form = match spec.kind {
        Kind::Container => make_form(e, 0, addr::FORM_TYPE_CONTAINER, 0),
        Kind::Actor => make_form(e, 0, FORM_TYPE_NPC, 0),
    };
    e.mem.set_u32(r + addr::REF_BASE_FORM_OFFSET, base_form);
    let container = base_form
        + match spec.kind {
            Kind::Container => addr::CONT_CONTAINER_OFFSET,
            Kind::Actor => addr::ACTOR_BASE_CONTAINER_OFFSET,
        };
    let inv = new(e, 8);
    let base: Vec<(u32, i32)> = spec.base.iter().map(|(id, n)| (item(e, *id), *n)).collect();
    let mut have: Vec<(u32, i32, Option<f32>)> = base.iter().map(|(f, n)| (*f, *n, None)).collect();
    for (id, n, condition) in &spec.items {
        have.push((item(e, *id), *n, *condition));
    }
    e.ext::<Kit>().holders.insert(
        r,
        Holder {
            base,
            have,
            inv,
            container,
            merchant: 0,
        },
    );
    rebuild(e, r);
}

/// Adds `spec`'s forms to `story` and the stand-ins for the game's item functions, and installs
/// co-op's item wraps in front of the ones it wraps.
pub fn install(story: &mut Story, spec: &ItemKitSpec) {
    let e = &mut story.e;
    e.map(IK, IK_LEN);
    e.ext::<Kit>().next = FIRST;
    for (vt, actor, is_item) in [
        (REF_VT, FALSE_FN, FALSE_FN),
        (ACTOR_VT, TRUE_FN, FALSE_FN),
        (ITEM_VT, FALSE_FN, TRUE_FN),
    ] {
        e.mem.set_u32(vt + addr::FORM_IS_ACTOR_SLOT, actor);
        e.mem.set_u32(vt + addr::FORM_IS_ITEM_SLOT, is_item);
        e.mem.set_u32(vt + addr::REF_REMOVE_ITEM_SLOT, REMOVE_FN);
    }
    e.register(TRUE_FN, |_, _| eax(1));
    e.register(FALSE_FN, |_, _| eax(0));
    e.register(REMOVE_FN, remove_fn);
    e.register(addr::REF_INVENTORY_CHANGES, |e, a| {
        eax(e.ext::<Kit>().holders.get(&a[0]).map_or(0, |h| h.inv))
    });
    e.register(addr::CHANGES_ITEM_COUNT, |e, a| {
        let form = a[1];
        eax(held(e, a[0], |_, f| f == form) as u32)
    });
    e.register(addr::CHANGES_GOLD_AMOUNT, |e, a| {
        eax(held(e, a[0], |e, f| {
            e.mem.u32(f + addr::FORM_ID_OFFSET) == addr::CAPS_FORM_ID
        }) as u32)
    });
    e.register(addr::EXTRA_GET_BY_TYPE, |e, a| {
        eax(match a[1] {
            addr::EXTRA_TYPE_COUNT => e.mem.u32(a[0] + XDATA_COUNT),
            addr::EXTRA_TYPE_HEALTH => e.mem.u32(a[0] + XDATA_HEALTH),
            _ => 0,
        })
    });
    e.register(addr::FORM_AS_HEALTH_FORM, |e, a| {
        let part = a[0] + HEALTH_PART;
        eax(if e.mem.u32(part + addr::HEALTH_FORM_HEALTH_OFFSET) != 0 {
            part
        } else {
            0
        })
    });
    e.register(addr::TEMP_CONTAINER_CONSTRUCT, |e, a| {
        e.ext::<Kit>().temps.insert(a[0], Vec::new());
        eax(a[0])
    });
    e.register(addr::TEMP_CONTAINER_ADD, |e, a| {
        if let Some(t) = e.ext::<Kit>().temps.get_mut(&a[0]) {
            t.push((a[1], a[2] as i32, None));
        }
        Ret::default()
    });
    e.register(addr::TEMP_CONTAINER_SET_HEALTH, |e, a| {
        if let Some(t) = e.ext::<Kit>().temps.get_mut(&a[0]) {
            for s in t.iter_mut() {
                s.2 = Some(f32::from_bits(a[1]));
            }
        }
        Ret::default()
    });
    e.register(addr::TEMP_CONTAINER_GIVE, |e, a| {
        let stacks = e.ext::<Kit>().temps.get(&a[0]).cloned().unwrap_or_default();
        for (form, count, condition) in stacks {
            give(e, a[1], form, count, condition);
        }
        Ret::default()
    });
    e.register(addr::TEMP_CONTAINER_DESTRUCT, |e, a| {
        e.ext::<Kit>().temps.remove(&a[0]);
        Ret::default()
    });
    e.register(addr::GET_MERCHANT_CONTAINER, |e, a| {
        let r = a[0].wrapping_sub(addr::REF_EXTRA_LIST_OFFSET);
        eax(e.ext::<Kit>().holders.get(&r).map_or(0, |h| h.merchant))
    });
    for at in [addr::CONTAINER_MENU_CREATE, addr::BARTER_MENU_CREATE] {
        e.register(at, |e, _| {
            e.ext::<Kit>().menus += 1;
            eax(1)
        });
    }
    e.register(addr::PLAYER_PICK_UP, |e, a| {
        let (player, r) = (a[0], a[1]);
        if e.mem.u32(r + addr::FORM_FLAGS_OFFSET) & addr::FORM_FLAG_DISABLED != 0 {
            return Ret::default();
        }
        let form = e.mem.u32(r + addr::REF_BASE_FORM_OFFSET);
        let count = e.mem.u32(r + WORLD_COUNT) as i32;
        give(e, player, form, count, None);
        let flags = e.mem.u32(r + addr::FORM_FLAGS_OFFSET);
        e.mem.set_u32(
            r + addr::FORM_FLAGS_OFFSET,
            flags | addr::FORM_FLAG_DISABLED,
        );
        add_change(e, r, addr::CHANGE_FORM_FLAGS);
        Ret::default()
    });
    items::install(e);

    let caps = ItemSpec {
        id: addr::CAPS_FORM_ID,
        health: 0,
    };
    for it in spec.items.iter().chain([&caps]) {
        if forms::form(e, it.id).is_some() {
            continue;
        }
        let f = make_form(e, ITEM_VT, FORM_TYPE_ITEM, it.id);
        e.mem
            .set_u32(f + HEALTH_PART + addr::HEALTH_FORM_HEALTH_OFFSET, it.health);
    }
    for h in &spec.holders {
        let (vt, kind) = match h.kind {
            Kind::Container => (REF_VT, addr::FORM_TYPE_REFR),
            Kind::Actor => (ACTOR_VT, addr::FORM_TYPE_ACHR),
        };
        let r = make_form(e, vt, kind, h.id);
        make_holder(e, r, h);
    }
    for h in &spec.holders {
        if h.merchant != 0 {
            let r = forms::form(e, h.id).expect("just made");
            let stock = forms::form(e, h.merchant).expect("the merchant container is a holder");
            if let Some(k) = e.ext::<Kit>().holders.get_mut(&r) {
                k.merchant = stock;
            }
        }
    }
    for w in &spec.world {
        let r = make_form(e, REF_VT, addr::FORM_TYPE_REFR, w.id);
        let form = item(e, w.item);
        e.mem.set_u32(r + addr::REF_BASE_FORM_OFFSET, form);
        e.mem.set_u32(r + WORLD_COUNT, w.count as u32);
    }
    if let Some(player) = live::player(e) {
        // The story's player: an actor that holds items, with the kit's slots in its own vtable.
        let vt = e.mem.u32(player);
        e.mem.set_u32(vt + addr::FORM_IS_ACTOR_SLOT, TRUE_FN);
        e.mem.set_u32(vt + addr::FORM_IS_ITEM_SLOT, FALSE_FN);
        e.mem.set_u32(vt + addr::REF_REMOVE_ITEM_SLOT, REMOVE_FN);
        e.mem
            .set_u8(player + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_ACHR);
        make_holder(
            e,
            player,
            &HolderSpec {
                kind: Kind::Actor,
                ..HolderSpec::default()
            },
        );
    }
}

fn reference(s: &mut Story, id: u32) -> u32 {
    forms::form(&mut s.e, id).expect("a reference of the kit")
}

/// The player opens the container menu on holder `id`.
pub fn open_container(s: &mut Story, id: u32) {
    let r = reference(s, id);
    s.e.call(addr::CONTAINER_MENU_CREATE, &args![r, 0u32]);
}

/// The player opens the barter menu on trader `id`.
pub fn open_barter(s: &mut Story, id: u32) {
    let r = reference(s, id);
    s.e.call(addr::BARTER_MENU_CREATE, &args![r, 0u32]);
}

/// The player picks placed item `id` up; whether it was still there.
pub fn pick_up(s: &mut Story, id: u32) -> bool {
    let there = items::in_world(&mut s.e, id);
    let r = reference(s, id);
    let player = live::player(&mut s.e).expect("a story with a player");
    s.e.call(addr::PLAYER_PICK_UP, &args![player, r, 1u32, 0u32]);
    there
}

/// Moves `count` of `item` from holder `from` to holder `to` as a menu does (the giver's removal
/// with the taker named); how many moved.
pub fn transfer(s: &mut Story, from: u32, to: u32, item_id: u32, count: i32) -> i32 {
    let (a, b) = (reference(s, from), reference(s, to));
    let form = item(&mut s.e, item_id);
    let before = items::count(&mut s.e, b, item_id);
    s.e.vcall(
        a,
        addr::REF_REMOVE_ITEM_SLOT,
        &args![
            form,
            0u32,
            count as u32,
            0u32,
            0u32,
            b,
            0u32,
            0u32,
            1u32,
            0u32
        ],
    );
    items::count(&mut s.e, b, item_id) - before
}

/// What holder `id` holds: a count per item form id.
pub fn holding(s: &mut Story, id: u32) -> BTreeMap<u32, i32> {
    let r = reference(s, id);
    items::totals(&mut s.e, r)
}

/// How many of `item` holder `id` holds.
pub fn has(s: &mut Story, id: u32, item_id: u32) -> i32 {
    let r = reference(s, id);
    items::count(&mut s.e, r, item_id)
}

/// The menus opened in this game (the stand-ins' calls).
pub fn menus_opened(s: &mut Story) -> u32 {
    s.e.ext::<Kit>().menus
}

/// The inventory and world changes the kit told the game's change journal of.
pub fn changes_seen(s: &mut Story) -> u32 {
    s.e.ext::<Kit>().changes
}
