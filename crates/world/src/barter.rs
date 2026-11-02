//! Trading with merchants (`ShowBarterMenu` on the speaker, as Trudy's
//! "Show me what you have for sale." line does), read from the game's
//! barter menu code (FalloutNV.exe, `0072d250` and the functions after it;
//! notes in `%USERPROFILE%\nv-re\findings\menus.md` §6):
//!
//! - The vendor's goods (`0046f310`): their own inventory and their
//!   merchant container's (the container their placed reference names in
//!   `XMRC`: Trudy's `00104DF3`, a chest out of sight), together; their caps
//!   the caps in both. What they'll trade in (`0072e6c0`): their `AIDT`
//!   services (u32 at 8): weapons and ammunition 0x1, armour 0x2, books
//!   0x8, misc items (and notes worth something) 0x400, aid by its equip
//!   type: chems 0x20, stimpaks 0x40, food 0x10, alcohol 0x4; never keys.
//! - An item's worth (`004bd400`): its value (`0048e8a0`; -1 without one) at
//!   its condition (`00647c00`): value × `fItemConditionValueBase` + (c /
//!   10)^`fItemConditionValueExp` × `fItemConditionValueMult` × (1 - base) ×
//!   value, c the condition in percent (10 without one), whole numbers from
//!   1 up; then rounded to tenths.
//! - The skill's multipliers (`00649050`): buying `fBarterBuyBase` + Barter ×
//!   `fBarterBuyMult` / 100, selling `fBarterSellBase` + Barter ×
//!   `fBarterSellMult` / 100 (1.55 - 0.45 and 0.45 + 0.45 in the game's
//!   data: 1.1 and 0.9 at Barter 100).
//! - A price (`0072ed00`): selling, worth × the selling multiplier (chips
//!   and caravan money × 1), × (1 + a/100) for a price adjustment a > 0 (1 -
//!   |a|/100 below 0), rounded (to tenths up to 1, else whole), at most the
//!   worth; buying, worth × the buying multiplier, × (1 - a/100) (1 +
//!   |a|/100 below 0), the player's perks' entry point 17 "Calculate Buy
//!   Price", rounded the same way, at least the worth; nothing worth less
//!   than 0 has a price (-1, shown "--").
//! - Accepting (`0072fd10`): the items bought go to the player, the items
//!   sold to the vendor's merchant container (else the vendor), the caps:
//!   the player pays what's owed (the total floored), or gets what they're
//!   owed up to the vendor's caps; `ITMBottlecapsDown` / `Up`; the misc statistic "Barter
//!   Amount Traded" counts it.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{game_setting, Facts, GameState};

const XMRC: FourCC = FourCC::new(b"XMRC");
const AIDT: FourCC = FourCC::new(b"AIDT");
const ENIT: FourCC = FourCC::new(b"ENIT");
const ETYP: FourCC = FourCC::new(b"ETYP");

/// Bottle caps: `Caps001`, form `0000000F` in the game.
pub fn caps(order: &LoadOrder) -> FormId {
    order.form_by_editor_id("Caps001").unwrap_or(FormId(0xF))
}

/// The container a merchant's goods are in (`XMRC` on their reference).
pub fn merchant_container(order: &LoadOrder, merchant: FormId) -> Option<FormId> {
    let rr = order.get(merchant)?;
    let record = rr.record().ok()?;
    let s = record.get(XMRC).filter(|s| s.data.len() >= 4)?;
    Some(rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
}

/// The holders a vendor trades from: themselves and their merchant
/// container (`0046f310`).
pub fn vendor_holders(order: &LoadOrder, vendor: FormId) -> Vec<FormId> {
    let mut out = vec![vendor];
    if let Some(c) = merchant_container(order, vendor) {
        out.push(c);
    }
    out
}

/// What sold items go into (`0072fd10`): the merchant container, else the
/// vendor.
pub fn sold_items_holder(order: &LoadOrder, vendor: FormId) -> FormId {
    merchant_container(order, vendor).unwrap_or(vendor)
}

fn barter_skill(order: &LoadOrder, state: &GameState) -> f32 {
    Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(PLAYER_REF, 32)
    .unwrap_or(0.0) as f32
}

/// The skill's multipliers for buying and selling (`00649050`).
pub fn multipliers(order: &LoadOrder, state: &GameState) -> (f32, f32) {
    let setting = |n: &str, default: f32| game_setting(order, n).unwrap_or(default);
    let skill = barter_skill(order, state);
    (
        setting("fBarterBuyBase", 1.5) + skill * setting("fBarterBuyMult", -0.5) / 100.0,
        setting("fBarterSellBase", 0.5) + skill * setting("fBarterSellMult", 0.5) / 100.0,
    )
}

/// Rounds as the game does (`004bd510`): to a multiple of `step`, the
/// fraction taken from the value divided by the step truncated, a half or
/// more going one step away from zero... for positive values; negative
/// ones are only truncated.
pub fn round_to(v: f32, step: f32) -> f32 {
    let q = v / step;
    let whole = q.trunc();
    let up = if q - whole >= 0.5 { 1.0 } else { 0.0 };
    (whole + up) * step
}

/// A value at a condition in percent (`00647c00`; -1: none, counted as
/// full).
pub fn value_at_condition(order: &LoadOrder, value: f32, condition: f32) -> f32 {
    let mut t = condition / 10.0;
    if t < 0.0 {
        t = 10.0;
    }
    let mut v = value;
    if value > 0.0 {
        let setting = |n: &str, default: f32| game_setting(order, n).unwrap_or(default);
        let base = setting("fItemConditionValueBase", 0.25);
        let mult = setting("fItemConditionValueMult", 0.03162);
        let exp = setting("fItemConditionValueExp", 1.5);
        v = value * base + t.powf(exp) * (mult * (1.0 - base) * value);
    }
    if v.abs() < 1.0 {
        v
    } else {
        round_to(v, 1.0)
    }
}

/// Whether a form has a value (`TESValueForm`: the kinds
/// [`base_value`] reads).
pub fn has_value(order: &LoadOrder, item: FormId) -> bool {
    order.get(item).is_some_and(|rr| {
        matches!(
            rr.entry.header.kind.as_bytes(),
            b"MISC"
                | b"KEYM"
                | b"IMOD"
                | b"WEAP"
                | b"ARMO"
                | b"CMNY"
                | b"CCRD"
                | b"AMMO"
                | b"BOOK"
                | b"ALCH"
                | b"INGR"
        )
    })
}

/// An item's value now: as `SetItemValue` set it, else as stored
/// ([`base_value`]).
pub fn value_now(order: &LoadOrder, state: &GameState, item: FormId) -> f32 {
    match state.more.item_values.get(&item) {
        Some(&v) => v as f32,
        None => base_value(order, item),
    }
}

/// An item's value as stored (`0048e8a0`): `DATA` (or `ENIT` for aid) of
/// the kinds that have one; -1 for the rest.
pub fn base_value(order: &LoadOrder, item: FormId) -> f32 {
    let Some(rr) = order.get(item) else {
        return -1.0;
    };
    let Ok(record) = rr.record() else {
        return -1.0;
    };
    let data = record
        .get(esm::sig::DATA)
        .map(|s| s.data.as_slice())
        .unwrap_or(&[]);
    let i32_at = |d: &[u8], at: usize| (d.len() >= at + 4).then(|| le_u32(d, at) as i32);
    let v = match rr.entry.header.kind.as_bytes() {
        b"MISC" | b"KEYM" | b"IMOD" | b"WEAP" | b"ARMO" | b"CMNY" | b"CCRD" => i32_at(data, 0),
        b"AMMO" => i32_at(data, 8),
        b"BOOK" => i32_at(data, 2),
        b"ALCH" | b"INGR" => record.get(ENIT).and_then(|s| i32_at(&s.data, 0)),
        _ => None,
    };
    v.map_or(-1.0, |v| v as f32)
}

/// What an item held is worth (`004bd400`): its value at its condition
/// (weapons whose condition scripts changed; everything else whole),
/// and its fitted mods' values (`world::weapon_mods`), rounded to tenths.
pub fn item_value(order: &LoadOrder, state: &GameState, holder: FormId, item: FormId) -> f32 {
    let condition = match state.weapon_health.get(&(holder, item)) {
        Some(&h) => (h * 100.0).min(100.0),
        None => {
            let kind = order.get(item).map(|r| r.entry.header.kind);
            if kind.is_some_and(|k| matches!(k.as_bytes(), b"WEAP" | b"ARMO")) {
                100.0
            } else {
                -1.0
            }
        }
    };
    let v = value_at_condition(order, value_now(order, state, item), condition)
        + crate::weapon_mods::worth_added(order, state, holder, item);
    round_to(v, 0.1)
}

/// A price for one of an item worth `worth` (`0072ed00`): `selling` from
/// the player, with the skill's `multipliers` and a price adjustment
/// (`ShowBarterMenu`'s, -100..100). -1 when it's worth less than nothing.
pub fn price(
    order: &LoadOrder,
    state: &GameState,
    item: FormId,
    worth: f32,
    selling: bool,
    multipliers: (f32, f32),
    adjustment: i32,
) -> f32 {
    if worth < 0.0 {
        return -1.0;
    }
    let money = order
        .get(item)
        .is_some_and(|r| matches!(r.entry.header.kind.as_bytes(), b"CHIP" | b"CMNY"));
    let a = adjustment.abs() as f32 / 100.0;
    let round = |p: f32| {
        if p.abs() <= 1.0 {
            round_to(p, 0.1)
        } else {
            round_to(p, 1.0)
        }
    };
    if selling {
        let mut p = worth * if money { 1.0 } else { multipliers.1 };
        if adjustment > 0 {
            p *= a + 1.0;
        } else if adjustment < 0 {
            p *= 1.0 - a;
        }
        round(p).min(worth)
    } else {
        let mut p = worth * if money { 1.0 } else { multipliers.0 };
        if adjustment > 0 {
            p *= 1.0 - a;
        } else if adjustment < 0 {
            p *= a + 1.0;
        }
        let p = crate::perks::apply(order, state, crate::perks::entry::CALCULATE_BUY_PRICE, p);
        round(p).max(worth)
    }
}

/// The vendor's services (`AIDT` u32 at 8, their template's when it gives
/// the AI data).
pub fn services(order: &LoadOrder, vendor: FormId) -> u32 {
    let base = crate::scripting::base_of(order, vendor).unwrap_or(vendor);
    crate::actor::data_record(order, base, crate::actor::USE_AI_DATA)
        .and_then(|(_, r)| {
            r.get(AIDT)
                .filter(|s| s.data.len() >= 12)
                .map(|s| le_u32(&s.data, 8))
        })
        .unwrap_or(0)
}

/// Whether a vendor with these services takes or sells an item
/// (`0072e6c0`).
pub fn deals_in(order: &LoadOrder, services: u32, item: FormId) -> bool {
    let Some(rr) = order.get(item) else {
        return false;
    };
    let flag = match rr.entry.header.kind.as_bytes() {
        b"ARMO" => 0x2,
        b"BOOK" => 0x8,
        b"MISC" | b"COBJ" | b"IMOD" => 0x400,
        b"WEAP" | b"AMMO" => 0x1,
        b"KEYM" => return false,
        b"ALCH" => {
            let equip = rr.record().ok().and_then(|r| {
                r.get(ETYP)
                    .filter(|s| s.data.len() >= 4)
                    .map(|s| le_u32(&s.data, 0) as i32)
            });
            match equip {
                Some(10) => 0x20,
                Some(11) => 0x40,
                Some(12) => 0x10,
                Some(13) => 0x4,
                _ => return true,
            }
        }
        b"NOTE" => {
            if base_value(order, item) < 1.0 {
                return false;
            }
            0x400
        }
        _ => return true,
    };
    services & flag != 0
}

/// A holder's caps.
fn caps_of(order: &LoadOrder, state: &GameState, holder: FormId) -> i32 {
    state.item_count(order, holder, caps(order))
}

/// The vendor's caps (`004cb320` over their goods: both holders).
pub fn vendor_caps(order: &LoadOrder, state: &GameState, vendor: FormId) -> i32 {
    vendor_holders(order, vendor)
        .into_iter()
        .map(|h| caps_of(order, state, h))
        .sum()
}

/// Takes `n` of an item from a vendor's holders (the merchant container
/// first — which the game takes first isn't traced) to someone.
fn take_from_vendor(
    order: &LoadOrder,
    state: &mut GameState,
    vendor: FormId,
    to: FormId,
    item: FormId,
    mut n: i32,
) -> i32 {
    let mut moved = 0;
    for holder in vendor_holders(order, vendor).into_iter().rev() {
        if n <= 0 {
            break;
        }
        let m = state.move_item(order, holder, to, item, n);
        n -= m;
        moved += m;
    }
    moved
}

/// Accepts a trade (`0072fd10`): `buys` (items and counts) go to the
/// player, `sells` from the player to the vendor's merchant container
/// (else the vendor), and `total` caps change hands (positive: the vendor
/// pays, up to their caps; negative: the player pays). Returns the sound
/// to play (`ITMBottlecapsUp` / `Down`), if any.
pub fn accept(
    order: &LoadOrder,
    state: &mut GameState,
    vendor: FormId,
    buys: &[(FormId, i32)],
    sells: &[(FormId, i32)],
    total: f32,
) -> Option<&'static str> {
    for &(item, n) in buys {
        take_from_vendor(order, state, vendor, PLAYER_REF, item, n);
    }
    let into = sold_items_holder(order, vendor);
    for &(item, n) in sells {
        let equipped = state.is_equipped(PLAYER_REF, item);
        state.move_item(order, PLAYER_REF, into, item, n);
        if equipped && state.item_count(order, PLAYER_REF, item) == 0 {
            state.unequip_item(order, PLAYER_REF, item);
        }
    }
    // The total as a whole number (`00406ce0`: floored).
    let caps_moved = total.floor() as i32;
    crate::stats::bump(state, BARTER_AMOUNT_TRADED, i64::from(caps_moved.abs()));
    let caps = caps(order);
    if total > 0.0 {
        let n = caps_moved.min(vendor_caps(order, state, vendor));
        take_from_vendor(order, state, vendor, PLAYER_REF, caps, n);
        Some("ITMBottlecapsUp")
    } else if total < 0.0 {
        state.move_item(order, PLAYER_REF, into, caps, -caps_moved);
        Some("ITMBottlecapsDown")
    } else {
        None
    }
}

/// The misc statistic "Barter Amount Traded" (number 39, `0072fd10`).
pub const BARTER_AMOUNT_TRADED: u8 = 39;

#[cfg(test)]
mod tests {
    use super::*;

    /// An empty game: only the exe's default settings.
    fn empty(tag: &str) -> (std::path::PathBuf, LoadOrder) {
        use testdata::{record, sub};
        let dir = std::env::temp_dir().join(format!("nv-rs-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let plugin = record(
            b"TES4",
            0,
            &sub(b"HEDR", &{
                let mut h = 1.34f32.to_le_bytes().to_vec();
                h.extend([0; 8]);
                h
            }),
        );
        std::fs::write(dir.join("FalloutNV.esm"), &plugin).unwrap();
        let order = LoadOrder::from_data_dir(&dir, &esm::ActivePlugins::OfficialOnly).unwrap();
        (dir, order)
    }

    /// `00647c00` with the exe's settings (base 0.25, mult 0.03162, exp
    /// 1.5): full condition keeps the value (99.996 rounds to 100); half
    /// condition 25 + 5^1.5 × 0.03162 × 0.75 × 100 = 51.5 → 52; below 1
    /// left as it is. `0072ed00`: selling at most the worth, buying at
    /// least it, small prices to tenths, -1 for nothing.
    #[test]
    fn values_and_prices() {
        let (dir, order) = empty("barter-prices");
        let state = GameState::default();
        assert_eq!(value_at_condition(&order, 100.0, 100.0), 100.0);
        assert_eq!(value_at_condition(&order, 100.0, -1.0), 100.0);
        assert_eq!(value_at_condition(&order, 100.0, 50.0), 52.0);
        assert!((value_at_condition(&order, 0.5, -1.0) - 0.5).abs() < 0.001);
        let m = (1.1, 0.9);
        let item = FormId(0x800);
        assert_eq!(price(&order, &state, item, 100.0, true, m, 0), 90.0);
        assert_eq!(price(&order, &state, item, 100.0, false, m, 0), 110.0);
        // A worse seller than the worth still sells at most for the worth.
        assert_eq!(
            price(&order, &state, item, 100.0, true, (1.1, 1.3), 0),
            100.0
        );
        assert_eq!(
            price(&order, &state, item, 100.0, false, (0.8, 0.9), 0),
            100.0
        );
        // A price adjustment of 20: selling × 1.2, buying × 0.8.
        assert_eq!(
            price(&order, &state, item, 100.0, true, (1.1, 0.5), 20),
            60.0
        );
        assert_eq!(
            price(&order, &state, item, 100.0, false, (1.5, 0.5), 20),
            120.0
        );
        assert!((price(&order, &state, item, 0.5, true, m, 0) - 0.5).abs() < 1e-6);
        assert_eq!(price(&order, &state, item, -1.0, true, m, 0), -1.0);
        let (b, s) = multipliers(&order, &state);
        assert_eq!((b, s), (1.5, 0.5));
        let _ = std::fs::remove_dir_all(dir);
    }

    /// `004bd510`: halves go up for positive values; negative ones are
    /// truncated toward zero.
    #[test]
    fn rounding_as_the_game_does() {
        assert_eq!(round_to(2.5, 1.0), 3.0);
        assert_eq!(round_to(2.49, 1.0), 2.0);
        assert_eq!(round_to(-2.7, 1.0), -2.0);
        assert!((round_to(0.46, 0.1) - 0.5).abs() < 1e-6);
    }
}
