//! What a script is and who owns it, read from the program. The guest gate decides by the
//! owner: the policy row of the owner quest says where a script runs.

use crate::addr;
use crate::program::Program;

/// A script's kind, from its type word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// An object's script.
    Object,
    /// A quest's script.
    Quest,
    /// A magic effect's script.
    Effect,
    /// Another type word.
    Other(u16),
}

/// One script as the program has it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Script {
    /// The script's form id. 0 for a script with no form (a console line, a result script).
    pub form: u32,
    /// Its kind.
    pub kind: Kind,
    /// The form id of the quest that owns it, when one does.
    pub quest: Option<u32>,
}

/// Reads the script at `script`.
pub fn read(p: &dyn Program, script: u32) -> Script {
    let mut word = [0; 2];
    p.read(script + addr::SCRIPT_TYPE, &mut word);
    let kind = match u16::from_le_bytes(word) {
        addr::script_type::OBJECT => Kind::Object,
        addr::script_type::QUEST => Kind::Quest,
        addr::script_type::EFFECT => Kind::Effect,
        other => Kind::Other(other),
    };
    let quest = match p.u32(script + addr::SCRIPT_QUEST) {
        0 => None,
        at => Some(p.u32(at + addr::FORM_ID)),
    };
    Script {
        form: p.u32(script + addr::FORM_ID),
        kind,
        quest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    #[test]
    fn a_quest_script_names_its_quest() {
        let mut p = FakeProgram::new();
        let quest = p.add_quest(0x0015_d912, &[10], &[]);
        p.set_u32(quest + addr::FORM_ID, 0x0015_d912);
        let script = 0x3000_0000;
        p.set_u32(script + addr::FORM_ID, 0x0015_d913);
        p.write(script + addr::SCRIPT_TYPE, &1u16.to_le_bytes());
        p.set_u32(script + addr::SCRIPT_QUEST, quest);
        assert_eq!(
            read(&p, script),
            Script {
                form: 0x0015_d913,
                kind: Kind::Quest,
                quest: Some(0x0015_d912),
            }
        );
    }

    #[test]
    fn an_object_script_and_an_effect_script_have_no_quest() {
        let mut p = FakeProgram::new();
        let object = 0x3000_0000;
        p.set_u32(object + addr::FORM_ID, 0x10);
        assert_eq!(
            read(&p, object),
            Script {
                form: 0x10,
                kind: Kind::Object,
                quest: None,
            }
        );
        let effect = 0x3000_0100;
        p.write(effect + addr::SCRIPT_TYPE, &0x100u16.to_le_bytes());
        assert_eq!(read(&p, effect).kind, Kind::Effect);
        p.write(effect + addr::SCRIPT_TYPE, &7u16.to_le_bytes());
        assert_eq!(read(&p, effect).kind, Kind::Other(7));
    }
}
