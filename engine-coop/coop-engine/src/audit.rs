//! The audit of a guest: each shared field against the host's last value.
//!
//! The gate stops what it knows. A write that it did not stop (an inline `set` of a script,
//! a command with no gate yet, a gate that sees nothing) leaves a guest's value different
//! from the host's. The audit finds each difference, names it, and gives the host's value
//! back. So a hole in the gate is loud, and it does not stay.
//!
//! The program itself moves some globals each frame (its clock). A guest's clock runs
//! between two values of the host: such a global is a leak only when it is farther from the
//! host's value than its limit.

use std::collections::BTreeMap;

use coop_core::ids::FormId;
use coop_core::schema::{Field, Key};

/// A shared slot whose value on this guest is not the host's.
#[derive(Clone, Debug, PartialEq)]
pub struct Leak {
    /// The form.
    pub form: FormId,
    /// The field's kind (`Field::kind`).
    pub kind: u8,
    /// The host's value. `None`: the host has no value in this slot.
    pub host: Option<Field>,
    /// This game's value. `None`: this game has no value in this slot.
    pub here: Option<Field>,
}

/// The host's last values, as a guest knows them.
#[derive(Clone, Debug, Default)]
pub struct Audit {
    host: BTreeMap<Key, (FormId, Field)>,
    known: bool,
    moving: BTreeMap<u32, f32>,
}

impl Audit {
    /// True after the host's first snapshot. Before it, there is nothing to compare with.
    pub fn known(&self) -> bool {
        self.known
    }

    /// The globals that the program moves itself (form ids), each with its limit: how far
    /// from the host's value a guest's value can be and not be a leak.
    pub fn set_moving(&mut self, globals: impl IntoIterator<Item = (u32, f32)>) {
        self.moving = globals.into_iter().collect();
    }

    /// The host sent a snapshot: these are all its values.
    pub fn snapshot(&mut self, entries: &[(FormId, Field)]) {
        self.host = entries
            .iter()
            .map(|(form, field)| (field.key(*form), (*form, field.clone())))
            .collect();
        self.known = true;
    }

    /// The host sent changes. A field with its default value clears its slot.
    pub fn delta(&mut self, changes: &[(FormId, Field)]) {
        for (form, field) in changes {
            let key = field.key(*form);
            if field.is_tombstone() {
                self.host.remove(&key);
            } else {
                self.host.insert(key, (*form, field.clone()));
            }
        }
    }

    fn within_limit(&self, form: FormId, host: &Field, here: &Field) -> bool {
        match (self.moving.get(&form.0), host, here) {
            (Some(limit), Field::Global { bits: a }, Field::Global { bits: b }) => {
                (f32::from_bits(*a) - f32::from_bits(*b)).abs() <= *limit
            }
            _ => false,
        }
    }

    /// Each slot of `now` (this game's export) that is not the host's value, and each slot
    /// of the host that `now` does not have. Sorted by key.
    pub fn leaks(&self, now: &[(FormId, Field)]) -> Vec<Leak> {
        let here: BTreeMap<Key, (FormId, &Field)> = now
            .iter()
            .map(|(form, field)| (field.key(*form), (*form, field)))
            .collect();
        let mut leaks = Vec::new();
        for (key, (form, field)) in &here {
            match self.host.get(key) {
                Some((_, host)) if host == *field => {}
                Some((_, host)) if self.within_limit(*form, host, field) => {}
                host => leaks.push(Leak {
                    form: *form,
                    kind: field.kind(),
                    host: host.map(|(_, f)| f.clone()),
                    here: Some((*field).clone()),
                }),
            }
        }
        for (key, (form, host)) in &self.host {
            if !here.contains_key(key) {
                leaks.push(Leak {
                    form: *form,
                    kind: host.kind(),
                    host: Some(host.clone()),
                    here: None,
                });
            }
        }
        leaks.sort_by_key(|l| (l.form, l.kind));
        leaks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const QUEST: FormId = FormId(0x0015_d912);
    const HOUR: FormId = FormId(0x38);

    fn host_values() -> Vec<(FormId, Field)> {
        vec![
            (QUEST, Field::QuestStage { stage: 10 }),
            (HOUR, Field::global(12.0)),
        ]
    }

    /// A guest with the host's values has no leak. A different stage, a slot that the host
    /// does not have, and a slot that the guest lost are each one leak with both values.
    #[test]
    fn a_difference_from_the_host_is_a_leak() {
        let mut audit = Audit::default();
        assert!(!audit.known());
        audit.snapshot(&host_values());
        assert!(audit.known());
        assert_eq!(audit.leaks(&host_values()), []);

        let other = FormId(0x0010_4c1c);
        let now = vec![
            (QUEST, Field::QuestStage { stage: 20 }),
            (other, Field::QuestStage { stage: 5 }),
        ];
        let leaks = audit.leaks(&now);
        assert_eq!(leaks.len(), 3, "{leaks:?}");
        assert_eq!(
            leaks[0],
            Leak {
                form: HOUR,
                kind: Field::global(0.0).kind(),
                host: Some(Field::global(12.0)),
                here: None,
            }
        );
        assert_eq!(leaks[1].form, other);
        assert_eq!(leaks[1].host, None);
        assert_eq!(leaks[2].form, QUEST);
        assert_eq!(leaks[2].host, Some(Field::QuestStage { stage: 10 }));
        assert_eq!(leaks[2].here, Some(Field::QuestStage { stage: 20 }));
    }

    /// The clock runs on a guest between two values of the host: inside its limit it is no
    /// leak, outside it is one. Mutation twin: with no limit, the same small step is a leak.
    #[test]
    fn a_moving_global_is_a_leak_only_outside_its_limit() {
        let mut audit = Audit::default();
        audit.snapshot(&host_values());
        let at = |hour: f32| {
            vec![
                (QUEST, Field::QuestStage { stage: 10 }),
                (HOUR, Field::global(hour)),
            ]
        };
        assert_eq!(audit.leaks(&at(12.01)).len(), 1, "the twin: no limit");
        audit.set_moving([(HOUR.0, 0.05)]);
        assert_eq!(audit.leaks(&at(12.01)), []);
        assert_eq!(audit.leaks(&at(11.99)), []);
        let leaks = audit.leaks(&at(8.0));
        assert_eq!(leaks.len(), 1);
        assert_eq!(leaks[0].form, HOUR);
    }

    /// A change from the host moves what the audit compares with, and a default value clears
    /// the slot.
    #[test]
    fn the_hosts_changes_move_the_reference() {
        let mut audit = Audit::default();
        audit.snapshot(&host_values());
        let running = Field::QuestFlags { flags: 1 };
        audit.delta(&[
            (QUEST, Field::QuestStage { stage: 20 }),
            (QUEST, running.clone()),
        ]);
        let now = vec![
            (QUEST, Field::QuestStage { stage: 20 }),
            (QUEST, running),
            (HOUR, Field::global(12.0)),
        ];
        assert_eq!(audit.leaks(&now), []);
        audit.delta(&[(QUEST, Field::QuestFlags { flags: 0 })]);
        let leaks = audit.leaks(&now);
        assert_eq!(leaks.len(), 1);
        assert_eq!(
            (leaks[0].form, leaks[0].kind, &leaks[0].host),
            (QUEST, 1, &None)
        );
    }
}
