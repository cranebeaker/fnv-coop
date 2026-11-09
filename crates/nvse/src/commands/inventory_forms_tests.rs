//! Tests of the batch through the commands' records, on forms laid out where the test's cast puts
//! their components and extra data lists answered by a stand-in of `BaseExtraList::GetByType`
//! (a list's data of type `t` at `+0x100 + 4t`, its presence bits at `+8`).

use super::super::test_support::*;
use super::*;

const ARMOR: u32 = 0x0200_1000;
const WEAPON: u32 = 0x0200_2000;
const AMMO: u32 = 0x0200_3000;
const REF: u32 = 0x0200_4000;
const ACTOR: u32 = 0x0200_5000;
const CHANGES: u32 = 0x0200_6000;
const XDATA: [u32; 3] = [0x0200_6400, 0x0200_6800, 0x0200_6c00];
const EXTRA: u32 = 0x0200_7000;
const NODES: u32 = 0x0200_7400;
const DESTRUCTIBLE: u32 = 0x0200_8000;

fn get_by_type(e: &mut Engine, a: &[u32]) -> Ret {
    Ret {
        eax: e.mem.u32(a[0] + 0x100 + 4 * a[1]),
        ..Ret::default()
    }
}

/// Marks `kind` present in a list with `data` as its extra data.
fn put_extra(e: &mut Engine, list: u32, kind: u32, data: u32) {
    let at = list + 8 + (kind >> 3);
    let bits = e.mem.u8(at) | (1 << (kind & 7));
    e.mem.set_u8(at, bits);
    e.mem.set_u32(list + 0x100 + 4 * kind, data);
}

/// An armor (slot 2, base health 100, flags 0x40), a weapon (non-playable flag), ammo, a
/// reference to the armor, and the script's references 1-3 naming them.
fn setup() -> Engine {
    let mut e = engine_with_table();
    e.register(0x0041_0220, get_by_type);
    e.mem.set_u8(ARMOR + 4, 0x18);
    e.mem.set_u32(ARMOR + 0x7f0, 1024 | 4);
    e.mem.set_u32(ARMOR + 0x700 + 4, 1 << 2);
    e.mem.set_u8(ARMOR + 0x700 + 8, 0x40);
    e.mem.set_u32(ARMOR + 0x200 + 4, 100);
    e.mem.set_u32(ARMOR + 0xc, 0x0001_0001);
    e.mem.set_u8(WEAPON + 4, 0x28);
    e.mem.set_u32(WEAPON + 0x7f0, 256 | 4);
    e.mem.set_u8(WEAPON + 0x100, 0x80);
    e.mem.set_u32(WEAPON + 0x200 + 4, 250);
    e.mem.set_u32(WEAPON + 0xc, 0x0001_0002);
    e.mem.set_u8(AMMO + 4, 0x29);
    e.mem.set_u32(AMMO + 0x7f0, 16);
    e.mem.set_u32(AMMO + 0xac, 2);
    e.mem.set_u8(REF + 4, 0x3a);
    e.mem.set_u32(REF + 0x7f0, 1);
    e.mem.set_u32(REF + 0x20, ARMOR);
    for (i, form) in [ARMOR, WEAPON, AMMO].into_iter().enumerate() {
        set_script_ref(&mut e, i as u16 + 1, form);
    }
    e
}

fn run(e: &mut Engine, opcode: u32, this: u32, args: &[Vec<u8>]) -> f64 {
    let (ok, result) = call(e, opcode, this, args);
    assert_eq!(ok, 1);
    result
}

/// `IsPlayable`: an armor's non-playable flag, a weapon's, ammo's; the caller's base without an
/// argument; `SetIsPlayable` 0 and the integer whose bits are -0.0 make it non-playable.
#[test]
fn playable_flags() {
    let mut e = setup();
    assert_eq!(run(&mut e, 0x15ac, 0, &[r(1)]), 0.0);
    assert_eq!(run(&mut e, 0x15ac, 0, &[r(2)]), 0.0);
    assert_eq!(run(&mut e, 0x15ac, 0, &[r(3)]), 0.0);
    assert_eq!(run(&mut e, 0x15ac, 0, &[]), 1.0, "no form");
    run(&mut e, 0x15ad, 0, &[n(1), r(1)]);
    assert_eq!(e.mem.u8(ARMOR + 0x708), 0);
    assert_eq!(run(&mut e, 0x15ac, REF, &[]), 1.0, "the caller's base");
    run(&mut e, 0x15ad, REF, &[n(i32::MIN)]);
    assert_eq!(e.mem.u8(ARMOR + 0x708), 0x40, "-0.0 is 0");
    run(&mut e, 0x15ad, 0, &[n(0), r(2)]);
    assert_eq!(e.mem.u8(WEAPON + 0x100), 0x80, "a weapon is left alone");
}

/// The slot mask keeps its 20 bits, the biped mask its low byte.
#[test]
fn slot_and_biped_masks() {
    let mut e = setup();
    assert_eq!(run(&mut e, 0x15ae, 0, &[r(1)]), 4.0);
    run(&mut e, 0x15af, 0, &[n(-1), r(1)]);
    assert_eq!(run(&mut e, 0x15ae, REF, &[]), f64::from(0xf_ffffu32));
    assert_eq!(run(&mut e, 0x15c5, 0, &[r(1)]), 64.0);
    run(&mut e, 0x15c6, REF, &[n(0x1a5)]);
    assert_eq!(e.mem.u8(ARMOR + 0x708), 0xa5);
    assert_eq!(run(&mut e, 0x15ae, 0, &[r(2)]), 0.0, "not a biped form");
}

/// An actor wearing the weapon (its second stack list worn; the first not) and the armor (worn
/// left), the weapon's worn list with an `ExtraHealth` of 37.5.
fn wearing(e: &mut Engine) {
    e.mem.set_u8(ACTOR + 4, 0x3b);
    e.mem.set_u32(ACTOR + 0x7f0, 1);
    put_extra(e, ACTOR + 0x44, 0x15, CHANGES);
    e.mem.set_u32(CHANGES + 0xc, CHANGES + 0x20);
    e.mem.set_u32(CHANGES + 0x20, NODES);
    // Entries: {extend list, count delta, item}; the entry list's nodes {data, next}.
    let (weapon_entry, armor_entry) = (NODES + 0x100, NODES + 0x110);
    e.mem.set_u32(NODES, weapon_entry);
    e.mem.set_u32(NODES + 4, NODES + 8);
    e.mem.set_u32(NODES + 8, armor_entry);
    e.mem.set_u32(weapon_entry, NODES + 0x40);
    e.mem.set_u32(weapon_entry + 8, WEAPON);
    e.mem.set_u32(NODES + 0x40, XDATA[0]);
    e.mem.set_u32(NODES + 0x44, NODES + 0x48);
    e.mem.set_u32(NODES + 0x48, XDATA[1]);
    e.mem.set_u32(armor_entry, NODES + 0x60);
    e.mem.set_u32(armor_entry + 8, ARMOR);
    e.mem.set_u32(NODES + 0x60, XDATA[2]);
    put_extra(e, XDATA[1], 0x16, 0);
    put_extra(e, XDATA[1], 0x25, EXTRA);
    e.mem.set_f32(EXTRA + 0xc, 37.5);
    put_extra(e, XDATA[2], 0x17, 0);
}

/// The equipped object of a slot (the weapon slot 5, the armor's slot 2, all slots for 20 and
/// up) and its health: the stack's `ExtraHealth`, else the base health; also as a condition.
#[test]
fn equipped_object_and_health() {
    let mut e = setup();
    wearing(&mut e);
    let (_, form) = call(&mut e, 0x1449, ACTOR, &[n(5)]);
    assert_eq!(form.to_bits() as u32, 0x0001_0002);
    let (_, form) = call(&mut e, 0x1449, ACTOR, &[n(2)]);
    assert_eq!(form.to_bits() as u32, 0x0001_0001);
    let (_, form) = call(&mut e, 0x1449, ACTOR, &[n(0)]);
    assert_eq!(form, 0.0);
    let (_, form) = call(&mut e, 0x1449, ACTOR, &[n(40)]);
    assert_eq!(form.to_bits() as u32, 0x0001_0002, "the first worn stack");
    assert_eq!(run(&mut e, 0x144a, ACTOR, &[n(5)]), 37.5);
    assert_eq!(run(&mut e, 0x144a, ACTOR, &[n(2)]), 100.0);
    assert_eq!(eval(&mut e, 0x144a, ACTOR, 5, 0), 37.5);
    assert_eq!(eval(&mut e, 0x144a, 0, 5, 0), 0.0);
    // A stack list after an empty node is not looked at.
    e.mem.set_u32(NODES + 0x40, 0);
    assert_eq!(run(&mut e, 0x144a, ACTOR, &[n(5)]), 0.0);
}

/// A reference's health: its `ExtraHealth`, else the base health made a float; a destructible
/// object's `ExtraObjectHealth`, else its data's; `IsEquipped` from the worn bits; `GetRefCount`
/// from `ExtraCount`, else 1.
#[test]
fn reference_health_worn_and_count() {
    let mut e = setup();
    assert_eq!(run(&mut e, 0x1451, REF, &[]), 100.0);
    put_extra(&mut e, REF + 0x44, 0x25, EXTRA);
    e.mem.set_f32(EXTRA + 0xc, 12.5);
    assert_eq!(run(&mut e, 0x1451, REF, &[]), 12.5);
    assert_eq!(eval(&mut e, 0x1451, 0, 0, 0), 0.0);
    e.mem.set_u32(DESTRUCTIBLE + 0x7f0, 1 << 19);
    e.mem.set_u32(DESTRUCTIBLE + 0x7e4, DESTRUCTIBLE + 0x100);
    e.mem.set_u32(DESTRUCTIBLE + 0x100, 40);
    e.mem.set_u32(ACTOR + 0x20, DESTRUCTIBLE);
    assert_eq!(run(&mut e, 0x1451, ACTOR, &[]), 40.0);
    put_extra(&mut e, ACTOR + 0x44, 0x56, EXTRA + 0x40);
    e.mem.set_f32(EXTRA + 0x4c, 7.0);
    assert_eq!(eval(&mut e, 0x1451, ACTOR, 0, 0), 7.0);
    assert_eq!(run(&mut e, 0x15b9, REF, &[]), 0.0);
    put_extra(&mut e, REF + 0x44, 0x17, 0);
    assert_eq!(run(&mut e, 0x15b9, REF, &[]), 1.0);
    assert_eq!(run(&mut e, 0x1476, REF, &[]), 1.0);
    put_extra(&mut e, REF + 0x44, 0x24, EXTRA + 0x80);
    e.mem.set_u16(EXTRA + 0x8c, 3);
    assert_eq!(run(&mut e, 0x1476, REF, &[]), 3.0);
}

/// The real game's answers (xNVSE 6.4.9, `real-tests/gr_q_inv.txt`, run `inv1`) on forms laid out
/// with the real ones' bytes: T-45d power armor (slots 4, flags 0xA0), the 9mm pistol and its
/// rounds (playable, no biped part), Leather Armor (slots 4, flags 0), then on Leather Armor
/// `SetIsPlayable 0`, `SetEquipmentSlotsMask -1`, `SetEquipmentBipedMask 421`.
#[test]
fn the_real_games_form_answers() {
    let mut e = setup();
    e.mem.set_u8(ARMOR + 0x708, 0xa0);
    e.mem.set_u8(WEAPON + 0x100, 0);
    e.mem.set_u32(AMMO + 0xac, 0);
    let line = |e: &mut Engine, at: u16| [0x15ac, 0x15ae, 0x15c5].map(|op| run(e, op, 0, &[r(at)]));
    assert_eq!(line(&mut e, 1), [1.0, 4.0, 160.0]);
    assert_eq!(line(&mut e, 2), [1.0, 0.0, 0.0]);
    assert_eq!(line(&mut e, 3), [1.0, 0.0, 0.0]);
    e.mem.set_u8(ARMOR + 0x708, 0);
    assert_eq!(line(&mut e, 1), [1.0, 4.0, 0.0]);
    run(&mut e, 0x15ad, 0, &[n(0), r(1)]);
    let playable = run(&mut e, 0x15ac, 0, &[r(1)]);
    run(&mut e, 0x15af, 0, &[n(-1), r(1)]);
    let slots = run(&mut e, 0x15ae, 0, &[r(1)]);
    run(&mut e, 0x15c6, 0, &[n(421), r(1)]);
    let biped = run(&mut e, 0x15c5, 0, &[r(1)]);
    assert_eq!([playable, slots, biped], [0.0, 1_048_575.0, 165.0]);
}

const HEAP: u32 = 0x0200_9000;
const REF_VTABLE: u32 = 0x0200_a000;

/// The form heap as a bump allocator, `BaseExtraList::Add` and `Remove` on the stand-in lists,
/// `MarkAsModified` recording its flags at `REF_VTABLE + 0x100`.
fn with_extra_writes(e: &mut Engine) {
    e.register(0x0040_1000, |e, a| {
        let at = e.mem.u32(HEAP).max(HEAP + 0x10);
        e.mem.set_u32(HEAP, at + a[0].next_multiple_of(16));
        Ret {
            eax: at,
            ..Ret::default()
        }
    });
    e.register(0x0040_ff60, |e, a| {
        let kind = u32::from(e.mem.u8(a[1] + 4));
        put_extra(e, a[0], kind, a[1]);
        Ret::default()
    });
    e.register(0x0041_0020, |e, a| {
        let kind = u32::from(e.mem.u8(a[1] + 4));
        let at = a[0] + 8 + (kind >> 3);
        let bits = e.mem.u8(at) & !(1 << (kind & 7));
        e.mem.set_u8(at, bits);
        e.mem.set_u32(a[0] + 0x100 + 4 * kind, 0);
        Ret::default()
    });
    e.mem.set_u32(REF, REF_VTABLE);
    e.mem.set_u32(REF_VTABLE + 0x48, 0x0060_0048);
    e.register(0x0060_0048, |e, a| {
        e.mem.set_u32(REF_VTABLE + 0x100, a[1]);
        Ret::default()
    });
}

/// `SetCurrentHealth`: below the base health an `ExtraHealth` is made, then changed; at or above
/// it the `ExtraHealth` goes; below 0 is 0. `SetEquippedCurrentHealth` the same on the worn stack.
#[test]
fn setting_health_makes_changes_and_drops_extra_health() {
    let mut e = setup();
    with_extra_writes(&mut e);
    let health = |e: &mut Engine| {
        let x = e.mem.u32(REF + 0x44 + 0x100 + 4 * 0x25);
        (x != 0).then(|| e.mem.f32(x + 0xc))
    };
    run(&mut e, 0x1452, REF, &[z(50.0)]);
    assert_eq!(health(&mut e), Some(50.0));
    run(&mut e, 0x1452, REF, &[z(-5.0)]);
    assert_eq!(health(&mut e), Some(0.0));
    run(&mut e, 0x1452, REF, &[z(100.0)]);
    assert_eq!(health(&mut e), None, "at the base health");
    wearing(&mut e);
    run(&mut e, 0x1450, ACTOR, &[z(20.0), n(5)]);
    assert_eq!(e.mem.f32(EXTRA + 0xc), 20.0, "the worn stack's ExtraHealth");
    run(&mut e, 0x1450, ACTOR, &[z(40.0), n(2)]);
    let made = e.mem.u32(XDATA[2] + 0x100 + 4 * 0x25);
    assert_eq!(e.mem.f32(made + 0xc), 40.0);
}

/// `SetRefCount` 1-0x7FFF: the reference's `ExtraCount` made, then changed, and the reference
/// marked (0x400); 0 and 0x8000 change nothing.
#[test]
fn set_ref_count_writes_extra_count() {
    let mut e = setup();
    with_extra_writes(&mut e);
    run(&mut e, 0x1477, REF, &[n(5)]);
    assert_eq!(run(&mut e, 0x1476, REF, &[]), 5.0);
    assert_eq!(e.mem.u32(REF_VTABLE + 0x100), 0x400);
    run(&mut e, 0x1477, REF, &[n(0x7fff)]);
    assert_eq!(run(&mut e, 0x1476, REF, &[]), 32767.0);
    run(&mut e, 0x1477, REF, &[n(0)]);
    run(&mut e, 0x1477, REF, &[n(0x8000)]);
    assert_eq!(run(&mut e, 0x1476, REF, &[]), 32767.0);
}

/// `CompareNames`: the base form's name (the second argument, else the caller's base) against the
/// first's, without case: -1/0/1; -2 when the base's name has no text; 0 when a form has no name.
#[test]
fn compare_names_orders_without_case() {
    let mut e = setup();
    let name = |e: &mut Engine, form: u32, text: &str| {
        let bits = e.mem.u32(form + 0x7f0) | (1 << 20);
        e.mem.set_u32(form + 0x7f0, bits);
        let at = form + 0x7a0;
        e.mem.write(at, text.as_bytes());
        e.mem
            .set_u32(form + 0x7d4, if text.is_empty() { 0 } else { at });
        e.mem.set_u16(form + 0x7d8, text.len() as u16);
    };
    name(&mut e, ARMOR, "Leather Armor");
    name(&mut e, WEAPON, "leather armor");
    assert_eq!(run(&mut e, 0x144b, 0, &[r(1), r(2)]), 0.0);
    name(&mut e, WEAPON, "Leather");
    assert_eq!(
        run(&mut e, 0x144b, 0, &[r(1), r(2)]),
        -1.0,
        "the base's is a prefix"
    );
    assert_eq!(run(&mut e, 0x144b, REF, &[r(2)]), 1.0, "the caller's base");
    name(&mut e, WEAPON, "");
    assert_eq!(run(&mut e, 0x144b, 0, &[r(1), r(2)]), -2.0);
    assert_eq!(
        run(&mut e, 0x144b, 0, &[r(3), r(1)]),
        0.0,
        "ammo has no name here"
    );
}

/// A container list that runs back into itself: the walk stops the game, or ends with the list
/// guard row.
#[test]
fn a_cyclic_list_stops_the_game_or_ends_the_walk() {
    let mut e = setup();
    wearing(&mut e);
    e.mem.set_u32(NODES + 0xc, NODES);
    let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        call(&mut e, 0x1449, ACTOR, &[n(0)]);
    }));
    assert!(stop.is_err());
    let mut e = setup();
    wearing(&mut e);
    e.mem.set_u32(NODES + 0xc, NODES);
    crate::commands::array::set_fix(&mut e, crate::commands::array::LIST_GUARD, true);
    let (_, form) = call(&mut e, 0x1449, ACTOR, &[n(0)]);
    assert_eq!(form, 0.0);
}
