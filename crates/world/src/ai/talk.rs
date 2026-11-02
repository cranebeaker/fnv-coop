//! A dialogue package's talk from one procedure update to the next, and
//! what becomes of the package once it has talked (FalloutNV.exe
//! 1.4.0.525; names marked (Xbox PDB) from the prototype's symbols).
//!
//! - The package's list is TRAVEL → DIALOGUE_ACTIVATE → WAIT → DIALOGUE →
//!   DONE (`011a3ff0` list 10). At DIALOGUE_ACTIVATE (`008e8600`) a
//!   dialogue package (type 15, not "Say To") only calls `InitiateDialogue`
//!   (Xbox PDB; actor +0x27c, `008b19c0`). That saves the running package
//!   with its procedure step (`SavePackageToExtraData` (Xbox PDB), process
//!   +0x710, `009130f0` → `0041c930`, extra data 0x19), installs a made
//!   conversation package (type 0x1c, `PutCreatedPackage` (Xbox PDB), actor
//!   +0x2f4, `0087eac0`) and puts it at its own DIALOGUE_ACTIVATE
//!   (`SetCurrentProcedureIndex(1)`, process +0x238). Nothing is said yet.
//! - On a later update the made package's DIALOGUE_ACTIVATE runs the
//!   ACTIVATE procedure (`008e9640`, process +0x7c8, called with 1): within
//!   reach and when the player can be force-greeted (`CanForceGreet` (Xbox
//!   PDB), process +0x3fc, `008da420`) the speaker activates the other
//!   (`00573170`).
//! - The activated person's base `Activate` (`005fa330`, an NPC's): when
//!   the activator's saved package is a dialogue package (type 15), it is
//!   saved again at step 4, its list's closing DONE (`0041c930(package, 4,
//!   target, 0, 0, 0)`), and finished (`PackageDone`, process +0x5a0: its
//!   end action; with "once a day" the day is noted, actor +0x28c). Then
//!   the dialogue menu (or the other's conversation) is asked for.
//! - When the menu closes (`00762160`) the speaker's `EndDialogue` (Xbox
//!   PDB; actor +0x288, `008b1070`) finds the made package current and
//!   restores the saved one at its saved step (`LoadPackageFromExtraData`
//!   (Xbox PDB), process +0x714, `00913250`; a step past the list's end
//!   would be 0): DONE, which list 10 doesn't step back from (`008eeec0`
//!   case 0x36). The close then asks for the speaker's package at once,
//!   twice (`CheckforNewPackage` (Xbox PDB), process +0x24, `008da670` →
//!   `0090a1a0`); when the list gives the same package again nothing
//!   changes (no new start, the step kept). So the package has talked once
//!   and stays done until a package starts anew (a different one is picked,
//!   or `0090a1a0` starts it again for its own reasons).
//!
//! Doc Mitchell's farewell (`VCG01DocMitchellFarewellDialogueStart`,
//! `GetStage VCG01 >= 115`) therefore talks once; its Goodbye line's
//! `VGenericTimer` then sets `VCG01` stage 200 (`Set GameHour to 8`, quest
//! completed), and the new hour has his sandbox picked.

/// What a dialogue package's DIALOGUE_ACTIVATE step does on this update.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TalkUpdate {
    /// `InitiateDialogue` (`008b19c0`): the conversation package is made;
    /// nothing is said on this update.
    Initiate,
    /// The made package's ACTIVATE (`008e9640`): the player is activated
    /// (the dialogue menu), or the other person's conversation begins.
    Activate,
}

/// Where a dialogue package stands between its procedure updates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DialogueRun {
    /// The TRAVEL step is over (the procedures only go on from there).
    pub travelled: bool,
    /// `InitiateDialogue` has made the conversation package.
    initiated: bool,
    /// The conversation (or the "Say To" line) is under way, or the
    /// package is done: nothing more from it until it starts anew.
    pub waiting: bool,
    /// It has talked: saved at DONE (`005fa330`), so it stays done after
    /// the conversation.
    pub done: bool,
}

impl DialogueRun {
    /// A package begins (`0090a1a0` → `StartNewPackage`, process +0x598):
    /// from its first step.
    pub fn begin() -> DialogueRun {
        DialogueRun::default()
    }

    /// The talk step on one procedure update: first the conversation
    /// package is made, the update after it activates. Activating finishes
    /// the package (`005fa330`); its end action is the caller's to ask for
    /// ([`crate::ai::actions::end`]).
    pub fn talk(&mut self) -> TalkUpdate {
        if self.initiated {
            self.initiated = false;
            self.waiting = true;
            self.done = true;
            TalkUpdate::Activate
        } else {
            self.initiated = true;
            TalkUpdate::Initiate
        }
    }

    /// Whether the made conversation package (type 0x1c) is theirs now,
    /// waiting for its ACTIVATE (what `009336c0` asks).
    pub fn conversation_package_made(&self) -> bool {
        self.initiated
    }

    /// The activation could not happen on this update (another talk opens
    /// first): the made package tries again on its next update.
    pub fn retry(&mut self) {
        self.initiated = true;
        self.waiting = false;
        self.done = false;
    }

    /// A "Say To" line was said: the GREET procedure ends the package
    /// (`008dbe30`, step 3).
    pub fn said(&mut self) {
        self.initiated = false;
        self.waiting = true;
        self.done = true;
    }

    /// The conversation is over (`EndDialogue`, `008b1070` → `00913250`):
    /// the saved package is back at its saved step, past its travel; after
    /// a talk that is DONE, so it stays waiting.
    pub fn conversation_over(&mut self) {
        self.travelled = true;
        self.initiated = false;
        self.waiting = self.done;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_menu_opens_one_update_after_the_conversation_package_is_made() {
        let mut run = DialogueRun::begin();
        assert_eq!(run.talk(), TalkUpdate::Initiate);
        assert!(!run.waiting);
        assert_eq!(run.talk(), TalkUpdate::Activate);
        assert!(run.waiting && run.done);
    }

    #[test]
    fn after_its_conversation_the_package_is_done() {
        let mut run = DialogueRun::begin();
        run.travelled = true;
        run.talk();
        run.talk();
        run.conversation_over();
        assert!(run.travelled && run.waiting && run.done);
        // Only a new start of the package talks again.
        let mut run = DialogueRun::begin();
        assert_eq!(run.talk(), TalkUpdate::Initiate);
    }

    #[test]
    fn a_blocked_activation_tries_again_on_the_next_update() {
        let mut run = DialogueRun::begin();
        run.talk();
        run.talk();
        run.retry();
        assert!(!run.done);
        assert_eq!(run.talk(), TalkUpdate::Activate);
    }
}
