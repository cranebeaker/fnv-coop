//! Package begin/end/change actions, read from `POBA`, `POEA`, `POCA`.
//!
//! FalloutNV.exe's package loader (`00673a40`) sends each marker to
//! `0067dd20`: `INAM` is the idle, the embedded script follows, and `TNAM`
//! is the topic and terminates the action. The three actions are independent.
//! For example, VCG01PlayerSection1 begins with the situp idle but changes
//! with the bedsit idle.
//!
//! When they run (FalloutNV.exe 1.4.0.525): the begin action (high-process
//! vfunc +0x598, `00903a80`) when `AddScriptPackage` installs a package
//! (`005cc4f0`) and when the package evaluation starts one (`0090a1a0`,
//! `00907ab0`); the end action (+0x5a0, `00903d10`) when a package's
//! procedures reach `DONE` (`0091ecf0`, once: the process flag +0x5a8)
//! and when a finished package gives way (`00907ab0`); the change action
//! (+0x59c, `00903bf0`) for the script package `AddScriptPackage` replaces
//! and on `ResetAI` (`008a6ce0`). Each runs, in order, the embedded
//! script on the actor (`005ac1e0`), the idle (process vfunc +0x44), and
//! the topic, said through the greeting procedure (vfunc +0x2a4) to no one
//! in particular.
//!
//! Not carried out: a begin action without an idle stops the actor's
//! current idle (+0x44 with none, unless `004985f0`), and an end action's
//! idle under package flag 0x4 sets process flag +0x590; whether the
//! evaluation begins a package again when it re-picks one that reached
//! `DONE` (`0090a1a0`'s branches) isn't followed: a package begins when it
//! changes.

use esm::{FormId, Record, Subrecord};

use crate::scripting::{Event, GameState, PackageActionKind, Runner};

/// One package action. Empty actions in game files have null INAM/TNAM and
/// an empty script header; retaining that header does not imply a script runs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageAction {
    pub idle: Option<FormId>,
    pub topic: Option<FormId>,
    /// Embedded script fields in file order. SCRO form references are
    /// remapped to the load order; SCRV variable indices and bytecode remain
    /// unchanged. Keep headers, locals and compiled code even without SCTX.
    pub script: Vec<Subrecord>,
}

impl PackageAction {
    pub fn source(&self) -> Option<String> {
        self.script
            .iter()
            .find(|s| s.kind.as_bytes() == b"SCTX")
            .map(Subrecord::zstring)
    }

    pub fn bytecode(&self) -> &[u8] {
        self.script
            .iter()
            .find(|s| s.kind.as_bytes() == b"SCDA")
            .map_or(&[], |s| s.data.as_slice())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageActions {
    pub begin: PackageAction,
    pub end: PackageAction,
    pub change: PackageAction,
}

impl PackageActions {
    pub(super) fn read(record: &Record, global: impl Fn(FormId) -> FormId) -> Self {
        let mut actions = Self::default();
        let mut active = None;
        let form = |data: &[u8]| {
            let raw = FormId(u32::from_le_bytes(data.get(..4)?.try_into().ok()?));
            // Null must stay null even for a plugin at a nonzero load index.
            (raw.0 != 0).then(|| global(raw))
        };
        for sub in &record.subrecords {
            match sub.kind.as_bytes() {
                b"POBA" | b"POEA" | b"POCA" => {
                    let action = match sub.kind.as_bytes() {
                        b"POBA" => &mut actions.begin,
                        b"POEA" => &mut actions.end,
                        _ => &mut actions.change,
                    };
                    // The original loader clears the action on entry.
                    *action = PackageAction::default();
                    active = Some(action);
                }
                b"TNAM" => {
                    if let Some(action) = active.take() {
                        action.topic = form(&sub.data);
                    }
                }
                b"INAM" => {
                    if let Some(action) = active.as_deref_mut() {
                        action.idle = form(&sub.data);
                    }
                }
                b"SCHR" | b"SCDA" | b"SCTX" | b"SLSD" | b"SCVR" | b"SCRO" | b"SCRV" => {
                    if let Some(action) = active.as_deref_mut() {
                        let mut field = sub.clone();
                        if sub.kind.as_bytes() == b"SCRO" && sub.data.len() >= 4 {
                            let id = form(&sub.data).unwrap_or(FormId(0));
                            field.data[..4].copy_from_slice(&id.0.to_le_bytes());
                        }
                        action.script.push(field);
                    }
                }
                _ => {}
            }
        }
        actions
    }
}

impl PackageActions {
    /// One of the three.
    pub fn get(&self, kind: PackageActionKind) -> &PackageAction {
        match kind {
            PackageActionKind::Begin => &self.begin,
            PackageActionKind::End => &self.end,
            PackageActionKind::Change => &self.change,
        }
    }
}

/// Asks for a package's begin action, as the AI starts the package for
/// `who` — unless it's the package whose begin was last asked for them
/// (`AddScriptPackage` begins and installs a package at once, `005cc4f0`,
/// so the AI taking it up doesn't begin it again). `restart`: the package
/// starts over (asked again even if the same).
pub fn begin(state: &mut GameState, who: FormId, package: FormId, restart: bool) {
    let same = state.package_begun.get(&who).map(|b| b.0) == Some(package);
    if package.0 == 0 || (!restart && same) {
        return;
    }
    state.package_begun.insert(who, (package, false));
    state.events.push(Event::PackageAction {
        who,
        package,
        kind: PackageActionKind::Begin,
    });
}

/// Asks for a package's end action: its procedures reached `DONE`
/// (`0091ecf0`; the low process's travel, `0090ad40`). Once per start
/// (process flag +0x5a8, cleared when a package begins, `0090a1a0`).
pub fn end(state: &mut GameState, who: FormId, package: FormId) {
    if package.0 == 0 || state.package_begun.get(&who) == Some(&(package, true)) {
        return;
    }
    state.package_begun.insert(who, (package, true));
    state.events.push(Event::PackageAction {
        who,
        package,
        kind: PackageActionKind::End,
    });
}

/// Carries out one action of a package for `who`: runs its embedded
/// script on them (with their own variables, as a dialogue line's result
/// script), and has them say its topic to no one in particular (a `SayTo`
/// with no listener, [`Event::Talk`]). Gives back its idle for the caller
/// to play (the player's go to the player's idle playback).
// Translated from 00903a80 / 00903d10 / 00903bf0 (decompiled, FalloutNV.exe
// 1.4.0.525): script, then idle, then topic.
pub fn perform(
    runner: &mut Runner,
    who: FormId,
    package: &super::Package,
    kind: PackageActionKind,
) -> Option<FormId> {
    let action = package.actions.get(kind);
    if let Some(source) = action.source().filter(|s| !s.trim().is_empty()) {
        runner.run_source(&source, Some(who), Some(who));
    }
    if let Some(topic) = action.topic {
        runner.state.events.push(Event::Talk {
            speaker: who,
            to: FormId(0),
            topic: Some(topic),
            conversation: false,
        });
    }
    action.idle
}
