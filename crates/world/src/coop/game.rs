//! The `world` adapter of the engine interface (`coop_game`).
//!
//! [`WorldCoop`] is a view over one `GameState` and the game data. It implements the traits
//! of `coop_game` with the functions that this module has already ([`super::export`],
//! [`super::apply`], [`super::backend`], [`Runner`], the load order). It adds no rule.
//!
//! Limits of this adapter:
//!
//! * The "nothing changed" answer of [`Shared::export_shared`] comes from the effect queue
//!   (`GameState::emit`). A writer that emits no effect does not show there. The caller
//!   must force an export at intervals, as `HostBridge::collect` does with its backstop.
//! * [`Shared::export_shared`] empties the effect queue. `HostBridge::collect` empties the
//!   same queue. Use only one of the two on a `GameState`.
//! * Only a host records effects, so [`Shared::activity`] does not move on a guest.

use coop_core::ids::FormId as CForm;
use coop_core::schema::{flag, Field};
use coop_game::{Applied, Change, Intent, Names, Refused, Role, Setup, Shared, Slot, Story};

use super::apply::{apply_changes, Ctx, Opts};
use super::backend::WorldBackend;
use super::export::{cf, domain_of, wf};
use super::{Authority, ShadowPlan};
use crate::scripting::{GameState, Runner};

/// One `GameState` with its game data, as a co-op session sees a game.
pub struct WorldCoop<'a> {
    gs: &'a mut GameState,
    ctx: Ctx<'a>,
}

impl<'a> WorldCoop<'a> {
    /// A view over `gs`. `ctx` is the load order and the script cache of the same game.
    pub fn new(gs: &'a mut GameState, ctx: Ctx<'a>) -> WorldCoop<'a> {
        WorldCoop { gs, ctx }
    }

    /// True when the policy gate holds this stage (`PolicyGate::pending`).
    fn held(&self, quest: esm::FormId, stage: u16) -> bool {
        self.gs.coop.gate.as_ref().is_some_and(|g| {
            g.pending()
                .iter()
                .any(|p| p.quest == quest && p.stage == stage)
        })
    }
}

impl Names for WorldCoop<'_> {
    fn form_of(&self, editor_id: &str) -> Option<CForm> {
        self.ctx.order.form_by_editor_id(editor_id).map(cf)
    }

    fn name_of(&self, form: CForm) -> Option<String> {
        self.ctx.order.get(wf(form))?.editor_id().ok().flatten()
    }

    fn load_index(&self, plugin: &str) -> Option<u8> {
        self.ctx
            .order
            .plugins()
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(plugin))
            .map(|p| p.load_index)
    }
}

impl Shared for WorldCoop<'_> {
    /// The authority changes only when it differs (`set_authority` drops the waiting
    /// effects). A host with a policy gets the gate and the shadow plan
    /// (`enable_coop_policy`). A guest with a policy gets the shadow plan (`enable_shadows`).
    /// Each of the two starts the shadow bank again.
    fn configure(&mut self, setup: &Setup) -> Result<(), String> {
        let want = match setup.role {
            Role::Host => Authority::Host,
            Role::Guest => Authority::Guest,
        };
        if self.gs.authority() != want {
            self.gs.set_authority(want);
        }
        if let Some(table) = &setup.policy {
            match setup.role {
                Role::Host => self.gs.enable_coop_policy(table.clone(), setup.me),
                Role::Guest => self
                    .gs
                    .enable_shadows(ShadowPlan::from_policy(table), setup.me),
            }
        }
        self.gs.coop.bank.hold_own = setup.keep_own_rows;
        Ok(())
    }

    /// `None` only on a host whose effect queue is empty. A guest and a game for one player
    /// record no effect, so they always give the list.
    fn export_shared(&mut self, force: bool) -> Option<Vec<(CForm, Field)>> {
        let gs = &mut *self.gs;
        let idle = gs.authority() == Authority::Host && gs.coop.out.is_empty() && !gs.coop.overflow;
        if idle && !force {
            return None;
        }
        gs.coop.out.clear();
        gs.coop.overflow = false;
        Some(gs.export_fields())
    }

    fn apply_shared(&mut self, changes: &[Change], present: bool) -> Applied {
        let opts = Opts {
            present,
            ctx: Some(self.ctx),
        };
        let report = apply_changes(self.gs, changes, opts);
        // `apply_changes` emits no effect. On a host these writes must show in the effect
        // queue, or the next export that is not forced gives no list.
        for (key, _) in changes {
            self.gs.emit(domain_of(key), wf(key.form));
        }
        Applied {
            set: report.set,
            cleared: report.cleared,
            announced: report.announced,
        }
    }

    fn quest(&self, quest: CForm) -> Option<(u16, u32)> {
        let gs = &*self.gs;
        let q = wf(quest);
        let stage = gs.stages.get(&q).copied();
        // The same bits as the export gives (`export::export`).
        let flags = [
            (gs.running.contains(&q), flag::RUNNING),
            (gs.completed.contains(&q), flag::COMPLETED),
            (gs.failed.contains(&q), flag::FAILED),
            (gs.quests_announced.contains(&q), flag::ANNOUNCED),
        ]
        .iter()
        .filter(|(on, _)| *on)
        .fold(0, |bits, (_, bit)| bits | bit);
        if stage.is_none() && flags == 0 {
            return None;
        }
        Some((stage.unwrap_or(0), flags))
    }

    fn activity(&self) -> u64 {
        self.gs.coop.stats.effects
    }
}

impl Story for WorldCoop<'_> {
    /// `SetStage` is `Runner::set_stage`. `ScriptLine` is `Runner::run_source` with no
    /// owner, as the console runs a line. Each runs inside `GameState::as_player(slot)`.
    ///
    /// A game for one player (authority `Single`) is not a host's game: it answers
    /// `NotHost`. A line that the script parser refuses gives `Ok(false)`.
    fn run_intent(&mut self, slot: Slot, intent: &Intent) -> Result<bool, Refused> {
        if self.gs.authority() != Authority::Host {
            return Err(Refused::NotHost);
        }
        let Ctx { order, scripts } = self.ctx;
        match intent {
            Intent::SetStage { quest, stage } => {
                let quest = wf(*quest);
                if scripts.quest(order, quest).is_none() {
                    return Err(Refused::NoSuchForm);
                }
                let ran = {
                    let mut live = self.gs.as_player(slot);
                    Runner::new(order, scripts, &mut live).set_stage(quest, *stage)
                };
                if !ran && self.held(quest, *stage) {
                    return Err(Refused::Held);
                }
                Ok(ran)
            }
            Intent::ScriptLine(line) => {
                if script::parse(line).is_err() {
                    return Ok(false);
                }
                let mut live = self.gs.as_player(slot);
                Runner::new(order, scripts, &mut live).run_source(line, None, None);
                Ok(true)
            }
        }
    }
}
