//! A player's own progress and ledgers, run on the host inside that
//! player's shadow (`GameState::with_player`): the level-up menu's choices,
//! a caravan game's result, a casino game's closing. Each is what the
//! viewer's menu does in single player; a guest's menu sends it as an
//! interaction act (`coop_core::interact::Act`) and the shadow replication
//! carries the result back.

use coop_core::interact::Why;
use esm::{FormId, LoadOrder};

use crate::casino::{Casino, CasinoData};
use crate::experience::LevelUp;
use crate::scripting::GameState;

/// The level-up menu closed (`world::experience::level_up`, then the
/// menu's skill clicks and the perk given on closing). Refused when no
/// level-up waits, a skill is not one the menu lists, the points are more
/// than the level gives, or the perk is not one the menu offers now.
pub fn level_up(
    order: &LoadOrder,
    gs: &mut GameState,
    skills: &[(u16, u32)],
    perk: FormId,
) -> Result<LevelUp, Why> {
    if !gs.level_up_pending {
        return Err(Why::Invalid);
    }
    if skills
        .iter()
        .any(|(av, _)| !crate::chargen::SHOWN_SKILLS.contains(av))
    {
        return Err(Why::Invalid);
    }
    // a tag skill's point raises it by `iSkillPointsTagSkillMult` (the menu's `tag_mult`)
    let mult = crate::scripting::game_setting(order, "iSkillPointsTagSkillMult")
        .map_or(1, |v| (v as u32).max(1));
    let spent: u64 = skills
        .iter()
        .map(|(av, n)| {
            let n = u64::from(*n);
            if gs.tag_skills.contains(av) {
                n.div_ceil(u64::from(mult))
            } else {
                n
            }
        })
        .sum();
    // all or nothing: what the level-up changes comes back on a refusal
    let before = (
        gs.player_level,
        gs.level_up_pending,
        gs.skill_points.clone(),
    );
    let up = crate::experience::level_up(order, gs);
    let offered = |gs: &GameState| {
        up.perk
            && crate::perks::level_up_choices(order, gs)
                .iter()
                .any(|c| c.perk == perk && c.available)
    };
    let refusal = if spent > u64::from(up.skill_points) {
        Some(Why::Insufficient)
    } else {
        for (av, n) in skills {
            let points = gs.skill_points.entry(*av).or_insert(0);
            *points = points.saturating_add(*n);
        }
        (perk.0 != 0 && !offered(gs)).then_some(Why::Policy)
    };
    if let Some(why) = refusal {
        (gs.player_level, gs.level_up_pending, gs.skill_points) = before;
        return Err(why);
    }
    if perk.0 != 0 {
        crate::perks::add(order, gs, perk);
    }
    Ok(up)
}

/// A caravan game ended (`PrepareResultsMenu`): the player's record, the
/// misc stat, the stake paid by the loser (`world::caravan::bet::settle`).
pub fn caravan_played(order: &LoadOrder, gs: &mut GameState, npc: FormId, stake: u32, won: bool) {
    let stake = i32::try_from(stake).unwrap_or(i32::MAX);
    crate::caravan::bet::settle(order, gs, npc, stake, won);
}

/// A casino game closed: the ledger the menu kept for that casino, then
/// the chips settled (`world::casino::settle`). Refused for a record that
/// is not a casino.
pub fn casino_closed(
    order: &LoadOrder,
    gs: &mut GameState,
    casino: FormId,
    chips: u32,
    winnings: i32,
    level: u8,
    new_level: bool,
) -> Result<(), Why> {
    let c = Casino::load(order, casino).ok_or(Why::Invalid)?;
    *crate::casino::data_mut(gs, casino) = CasinoData {
        casino,
        winnings,
        level: i16::from(level.min(4)),
    };
    let chips = i32::try_from(chips).unwrap_or(i32::MAX);
    crate::casino::settle(order, gs, &c, chips, new_level);
    Ok(())
}
