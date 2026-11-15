//! The guest gate: where the story's code runs.
//!
//! The host's game runs the story once. A guest's game runs each script that the policy makes
//! local, and each script that no quest owns (an object's or an effect's script: its menus,
//! sounds and messages are the guest's own). A script of a shared quest does not run on a
//! guest. While any script runs on a guest, a change of a shared quest's state does nothing
//! there: it becomes an intent, and the host decides and runs it.
//!
//! This module is the decision and the intent queue. It reads nothing from the program: the
//! glue reads the script ([`crate::script`]) and asks here.

use std::collections::BTreeSet;

use crate::script::{Kind, Script};

/// What the campaign policy says about where story code runs, for one load order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Rules {
    /// The quests that each player runs alone (form ids): the policy's per-player quests.
    pub local_quests: BTreeSet<u32>,
    /// The scripts that a guest runs itself (form ids): the policy's `guest_local` lists.
    pub local_scripts: BTreeSet<u32>,
}

/// Where a piece of story code runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Here, on the guest.
    Here,
    /// Not here: the host's game runs it.
    Host,
}

impl Rules {
    /// Whether each player runs this quest alone.
    pub fn quest_is_local(&self, quest: u32) -> bool {
        self.local_quests.contains(&quest)
    }

    /// Where a script runs on a guest. `own`: the adapter started this run itself (its
    /// console line).
    pub fn script(&self, script: &Script, own: bool) -> Verdict {
        if own || (script.form != 0 && self.local_scripts.contains(&script.form)) {
            return Verdict::Here;
        }
        match (script.quest, script.kind) {
            (Some(quest), _) if self.quest_is_local(quest) => Verdict::Here,
            (Some(_), _) => Verdict::Host,
            // A quest's script with no owner to ask the policy about stays closed.
            (None, Kind::Quest) => Verdict::Host,
            (None, _) => Verdict::Here,
        }
    }

    /// Where a change of this quest's state happens on a guest. `own`: the adapter makes the
    /// change itself.
    pub fn quest_change(&self, quest: u32, own: bool) -> Verdict {
        if own || self.quest_is_local(quest) {
            Verdict::Here
        } else {
            Verdict::Host
        }
    }
}

/// A change of a shared quest that a guest's game wanted to make. The host decides.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Intent {
    /// `SetStage`.
    Stage {
        /// The quest's form id.
        quest: u32,
        /// The stage.
        stage: u8,
    },
    /// The quest starts or stops.
    Running {
        /// The quest's form id.
        quest: u32,
        /// Start (true) or stop.
        on: bool,
    },
    /// The quest is completed, or not completed again.
    Completed {
        /// The quest's form id.
        quest: u32,
        /// Completed (true).
        on: bool,
    },
    /// The quest failed, or not failed again.
    Failed {
        /// The quest's form id.
        quest: u32,
        /// Failed (true).
        on: bool,
    },
    /// An objective's state.
    Objective {
        /// The quest's form id.
        quest: u32,
        /// The objective's id.
        objective: u32,
        /// Bit 0 displayed, bit 1 completed.
        state: u8,
    },
}

/// The intents that wait for the host, each one time. A guest's script that the guard
/// stopped can ask again each frame (its own "done" flag never sets): the queue keeps one.
#[derive(Clone, Debug, Default)]
pub struct Intents {
    waiting: Vec<Intent>,
    /// How many times a script asked for a change that the guard stopped.
    pub asked: u64,
}

impl Intents {
    /// Notes an intent. An intent that waits already is not added again.
    pub fn push(&mut self, intent: Intent) {
        self.asked += 1;
        if !self.waiting.contains(&intent) {
            self.waiting.push(intent);
        }
    }

    /// The intents that wait, in the order they came. The queue is empty after.
    pub fn take(&mut self) -> Vec<Intent> {
        std::mem::take(&mut self.waiting)
    }

    /// How many intents wait.
    pub fn len(&self) -> usize {
        self.waiting.len()
    }

    /// True when no intent waits.
    pub fn is_empty(&self) -> bool {
        self.waiting.is_empty()
    }
}

/// What the gate counted, for the checks that make it fail loudly.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counters {
    /// Script runs that reached the gate.
    pub runs: u64,
    /// Script runs that the gate left to the host.
    pub stopped: u64,
    /// Changes of a shared quest that the guard turned into an intent.
    pub guarded: u64,
    /// The form id of the script that reached the gate last (0: a script with no form).
    pub last_script: u32,
    /// How long the game played with no script at the gate, in milliseconds.
    pub quiet_ms: u32,
}

/// The longest time of play with no script at the gate. A game runs scripts each frame: a
/// gate that sees none for this long is not in the script path any more.
pub const BLIND_MS: u32 = 5000;

/// One guest's gate: the rules, the adapter's own bracket, the counters and the intents.
#[derive(Clone, Debug, Default)]
pub struct Gate {
    /// The policy's rules for this load order.
    pub rules: Rules,
    /// The intents that wait for the host.
    pub intents: Intents,
    /// What the gate counted.
    pub counters: Counters,
    /// How deep the adapter is in its own calls into the program.
    own: u32,
    /// How deep the adapter is in applying another machine's state.
    apply: u32,
    /// The run count at the last frame.
    runs_seen: u64,
}

impl Gate {
    /// A gate with these rules.
    pub fn new(rules: Rules) -> Gate {
        Gate {
            rules,
            ..Gate::default()
        }
    }

    /// The adapter starts a call of its own into the program (a console line, an apply).
    /// What the program runs inside it is not gated.
    pub fn enter_own(&mut self) {
        self.own += 1;
    }

    /// The adapter's own call ended.
    pub fn leave_own(&mut self) {
        self.own = self.own.saturating_sub(1);
    }

    /// The adapter starts to apply another machine's state (a quest's objectives, an actor's
    /// death). Inside, no script runs: the host ran them, and their effects come as state.
    /// The adapter's own change of a quest passes. It holds inside the adapter's own call too.
    pub fn enter_apply(&mut self) {
        self.apply += 1;
    }

    /// The apply ended.
    pub fn leave_apply(&mut self) {
        self.apply = self.apply.saturating_sub(1);
    }

    /// One frame of play passed, this many milliseconds long.
    pub fn frame(&mut self, play_ms: u32) {
        if self.counters.runs != self.runs_seen {
            self.runs_seen = self.counters.runs;
            self.counters.quiet_ms = 0;
        } else {
            self.counters.quiet_ms = self.counters.quiet_ms.saturating_add(play_ms);
        }
    }

    /// True when the gate saw no script for [`BLIND_MS`] of play: it protects nothing, and
    /// the session must say so.
    pub fn blind(&self) -> bool {
        self.counters.quiet_ms >= BLIND_MS
    }

    /// Starts the counters and the intent queue again (after the start-up checks).
    pub fn clear_counts(&mut self) {
        self.counters = Counters::default();
        self.runs_seen = 0;
        self.intents = Intents::default();
    }

    /// A script reached the runner. Where does it run?
    pub fn script(&mut self, script: &Script) -> Verdict {
        self.counters.runs += 1;
        self.counters.last_script = script.form;
        let verdict = if self.apply > 0 {
            Verdict::Host
        } else {
            self.rules.script(script, self.own > 0)
        };
        if verdict == Verdict::Host {
            self.counters.stopped += 1;
        }
        verdict
    }

    /// The program wants to change a quest's state. A change that goes to the host waits as
    /// `intent`.
    pub fn quest_change(&mut self, quest: u32, intent: Intent) -> Verdict {
        let verdict = self
            .rules
            .quest_change(quest, self.own > 0 || self.apply > 0);
        if verdict == Verdict::Host {
            self.counters.guarded += 1;
            self.intents.push(intent);
        }
        verdict
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHARED: u32 = 0x0015_d912;
    const LOCAL: u32 = 0x0010_4c1c;
    const LISTED: u32 = 0x0007_691e;

    fn rules() -> Rules {
        Rules {
            local_quests: [LOCAL].into(),
            local_scripts: [LISTED].into(),
        }
    }

    fn script(form: u32, kind: Kind, quest: Option<u32>) -> Script {
        Script { form, kind, quest }
    }

    /// A shared quest's script goes to the host. A local quest's script, a listed script and
    /// an object's or effect's script run here. Mutation twin: with empty rules the local
    /// quest's script goes to the host too.
    #[test]
    fn the_owner_decides_where_a_script_runs() {
        let r = rules();
        let quest = |q| script(0x100, Kind::Quest, Some(q));
        assert_eq!(r.script(&quest(SHARED), false), Verdict::Host);
        assert_eq!(r.script(&quest(LOCAL), false), Verdict::Here);
        assert_eq!(
            r.script(&script(LISTED, Kind::Quest, Some(SHARED)), false),
            Verdict::Here,
            "a listed script of a shared quest"
        );
        assert_eq!(
            r.script(&script(0, Kind::Object, Some(SHARED)), false),
            Verdict::Host,
            "a stage result of a shared quest"
        );
        for kind in [Kind::Object, Kind::Effect, Kind::Other(7)] {
            assert_eq!(r.script(&script(0x200, kind, None), false), Verdict::Here);
        }
        assert_eq!(
            r.script(&script(0x300, Kind::Quest, None), false),
            Verdict::Host,
            "a quest script with no owner stays closed"
        );
        assert_eq!(Rules::default().script(&quest(LOCAL), false), Verdict::Host);
    }

    /// The adapter's own line runs whatever it is, and a form id of 0 is never "listed".
    #[test]
    fn the_adapters_own_run_passes_and_no_form_is_not_listed() {
        let r = Rules {
            local_scripts: [0].into(),
            ..Rules::default()
        };
        let shared = script(0, Kind::Quest, Some(SHARED));
        assert_eq!(r.script(&shared, false), Verdict::Host);
        assert_eq!(r.script(&shared, true), Verdict::Here);
    }

    #[test]
    fn a_shared_quests_state_changes_on_the_host() {
        let r = rules();
        assert_eq!(r.quest_change(SHARED, false), Verdict::Host);
        assert_eq!(r.quest_change(LOCAL, false), Verdict::Here);
        assert_eq!(r.quest_change(SHARED, true), Verdict::Here);
    }

    /// A script that asks each frame gives one intent, and the count of asks stays honest.
    #[test]
    fn an_intent_waits_one_time() {
        let mut intents = Intents::default();
        let stage = Intent::Stage {
            quest: SHARED,
            stage: 20,
        };
        for _ in 0..53 {
            intents.push(stage);
        }
        intents.push(Intent::Running {
            quest: SHARED,
            on: true,
        });
        assert_eq!((intents.len(), intents.asked), (2, 54));
        assert_eq!(intents.take()[0], stage);
        assert!(intents.is_empty());
        intents.push(stage);
        assert_eq!(
            intents.len(),
            1,
            "after the host took it, a new ask waits again"
        );
    }

    /// The gate counts each run, stops a shared quest's script and turns a shared quest's
    /// change into one intent. Inside the adapter's bracket nothing is stopped, and after the
    /// bracket the gate stops again (the twin of the bracket: left open, the script would run).
    #[test]
    fn the_gate_counts_stops_and_brackets() {
        let mut gate = Gate::new(rules());
        let shared = script(0x100, Kind::Quest, Some(SHARED));
        let stage = Intent::Stage {
            quest: SHARED,
            stage: 10,
        };
        assert_eq!(gate.script(&shared), Verdict::Host);
        assert_eq!(gate.quest_change(SHARED, stage), Verdict::Host);
        assert_eq!(gate.quest_change(SHARED, stage), Verdict::Host);
        assert_eq!(gate.quest_change(LOCAL, stage), Verdict::Here);
        gate.enter_own();
        assert_eq!(gate.script(&shared), Verdict::Here);
        assert_eq!(gate.quest_change(SHARED, stage), Verdict::Here);
        gate.leave_own();
        assert_eq!(gate.script(&shared), Verdict::Host, "the bracket closed");
        gate.leave_own();
        assert_eq!(
            gate.script(&shared),
            Verdict::Host,
            "an extra close is harmless"
        );
        assert_eq!(
            gate.counters,
            Counters {
                runs: 4,
                stopped: 3,
                guarded: 2,
                last_script: 0x100,
                quiet_ms: 0,
            }
        );
        assert_eq!(gate.intents.take(), [stage]);
    }

    /// While the adapter applies another machine's state, no script runs, not even inside its
    /// own call, and its own quest change passes. After the apply the own call runs scripts
    /// again. Mutation twin: outside the apply, the same change of a shared quest is held.
    #[test]
    fn an_apply_runs_no_script_and_its_change_passes() {
        let mut gate = Gate::new(rules());
        let object = script(0x200, Kind::Object, None);
        let objective = Intent::Objective {
            quest: SHARED,
            objective: 10,
            state: 1,
        };
        assert_eq!(
            gate.quest_change(SHARED, objective),
            Verdict::Host,
            "the twin: no apply"
        );
        gate.enter_own();
        gate.enter_apply();
        assert_eq!(gate.script(&object), Verdict::Host);
        assert_eq!(
            gate.script(&script(0x100, Kind::Quest, Some(LOCAL))),
            Verdict::Host
        );
        assert_eq!(gate.quest_change(SHARED, objective), Verdict::Here);
        gate.leave_apply();
        assert_eq!(gate.script(&object), Verdict::Here, "the own call again");
        gate.leave_own();
        gate.leave_apply();
        assert_eq!(
            gate.script(&object),
            Verdict::Here,
            "an extra close is harmless"
        );
        assert_eq!(gate.intents.take(), [objective]);
        assert_eq!(gate.counters.stopped, 2);
    }

    /// A gate that sees a script each frame is not blind. A gate that sees none for five
    /// seconds of play is blind, and one script ends the alarm.
    #[test]
    fn a_gate_with_no_script_for_five_seconds_is_blind() {
        let mut gate = Gate::new(rules());
        let object = script(0x200, Kind::Object, None);
        for _ in 0..600 {
            gate.script(&object);
            gate.frame(16);
        }
        assert!(!gate.blind(), "the twin: scripts reach the gate");
        for _ in 0..312 {
            gate.frame(16);
        }
        assert!(!gate.blind(), "{} ms", gate.counters.quiet_ms);
        gate.frame(16);
        assert!(gate.blind());
        gate.script(&object);
        gate.frame(16);
        assert!(!gate.blind());
        gate.clear_counts();
        assert_eq!(gate.counters, Counters::default());
    }
}
