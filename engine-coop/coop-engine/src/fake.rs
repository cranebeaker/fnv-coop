//! A fake game program for unit tests: a byte map with quests and globals laid out as the
//! program lays them out ([`crate::addr`]), and the two functions the adapter calls.

use std::collections::BTreeMap;

use crate::addr;
use crate::program::{Program, Ret};

/// A fake program. Memory that was never written reads as zero.
#[derive(Default)]
pub struct FakeProgram {
    mem: BTreeMap<u32, u8>,
    /// Form id to address, as the program's form map.
    forms: BTreeMap<u32, u32>,
    /// The next free address.
    next: u32,
    /// Each call: the address and its arguments.
    pub calls: Vec<(u32, Vec<u32>)>,
    /// The quests whose stage `SetStage` set, in order: (quest address, stage). The fake runs
    /// no result script, so a test can check that a call happened.
    pub stages_set: Vec<(u32, u8)>,
    /// The blocks from [`Program::alloc`] that were not freed.
    pub live: Vec<u32>,
    /// What the console manager's getter answers (0: no manager yet).
    pub console_manager: u32,
}

impl FakeProgram {
    /// An empty program.
    pub fn new() -> FakeProgram {
        FakeProgram {
            next: 0x2000_0000,
            ..Default::default()
        }
    }

    fn take(&mut self, len: u32) -> u32 {
        let at = self.next;
        self.next += len.max(1).next_multiple_of(16);
        at
    }

    /// Makes a list (head node inline at `head`) of these items, as the program's lists are.
    fn link(&mut self, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            self.set_u32(node + addr::NODE_ITEM, *item);
            let next = if i + 1 < items.len() { self.take(8) } else { 0 };
            self.set_u32(node + addr::NODE_NEXT, next);
            node = next;
        }
    }

    /// Adds a quest with these stage numbers and objective ids. Returns its address.
    pub fn add_quest(&mut self, id: u32, stages: &[u8], objectives: &[u32]) -> u32 {
        let quest = self.take(0x80);
        let stage_items: Vec<u32> = stages
            .iter()
            .map(|n| {
                let item = self.take(16);
                self.set_u8(item + addr::STAGE_NUMBER, *n);
                item
            })
            .collect();
        self.link(quest + addr::QUEST_STAGES, &stage_items);
        let objective_items: Vec<u32> = objectives
            .iter()
            .map(|id| {
                let item = self.take(0x30);
                self.set_u32(item + addr::OBJECTIVE_ID, *id);
                item
            })
            .collect();
        self.link(quest + addr::QUEST_OBJECTIVES, &objective_items);
        self.forms.insert(id, quest);
        quest
    }

    /// Gives the quest at `quest` an event list with these variable ids, each at 0. Returns
    /// the variables' addresses.
    pub fn add_variables(&mut self, quest: u32, ids: &[u32]) -> Vec<u32> {
        let events = self.take(0x10);
        let head = self.take(8);
        let items: Vec<u32> = ids
            .iter()
            .map(|id| {
                let item = self.take(16);
                self.set_u32(item + addr::VAR_ID, *id);
                item
            })
            .collect();
        self.link(head, &items);
        self.set_u32(events + addr::EVENT_LIST_VARS, head);
        self.set_u32(quest + addr::QUEST_EVENT_LIST, events);
        items
    }

    /// Gives the quest at `quest` its own script with this form id. Returns the script's
    /// address.
    pub fn add_script(&mut self, quest: u32, id: u32) -> u32 {
        let script = self.take(addr::SCRIPT_BYTES);
        self.set_u32(script + addr::FORM_ID, id);
        self.set_u32(
            script + addr::SCRIPT_TYPE,
            u32::from(addr::script_type::QUEST),
        );
        self.set_u32(script + addr::SCRIPT_QUEST, quest);
        self.set_u32(quest + addr::QUEST_SCRIPT, script);
        self.forms.insert(id, script);
        script
    }

    /// Adds a global with this value. Returns its address.
    pub fn add_global(&mut self, id: u32, value: f32) -> u32 {
        let global = self.take(0x28);
        self.set_f32(global + addr::GLOBAL_VALUE, value);
        self.forms.insert(id, global);
        global
    }

    /// The program's `TESQuest::SetStage`, without the result scripts: the stage must exist,
    /// it becomes done, the quest runs, and the current stage only goes up.
    fn set_stage(&mut self, quest: u32, stage: u8) -> bool {
        let items = crate::quest::stage_items(self, quest);
        let Some((item, _)) = items.into_iter().find(|(_, n)| *n == stage) else {
            return false;
        };
        self.set_u8(item + addr::STAGE_DONE, 1);
        let flags = self.u8(quest + addr::QUEST_FLAGS) | addr::quest_flag::RUNNING;
        self.set_u8(quest + addr::QUEST_FLAGS, flags);
        if self.u8(quest + addr::QUEST_STAGE) < stage {
            self.set_u8(quest + addr::QUEST_STAGE, stage);
        }
        self.stages_set.push((quest, stage));
        true
    }
}

impl Program for FakeProgram {
    fn read(&self, at: u32, out: &mut [u8]) {
        for (i, b) in out.iter_mut().enumerate() {
            *b = self.mem.get(&(at + i as u32)).copied().unwrap_or(0);
        }
    }

    fn write(&mut self, at: u32, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            self.mem.insert(at + i as u32, *b);
        }
    }

    fn alloc(&mut self, len: u32) -> u32 {
        let at = self.take(len);
        self.live.push(at);
        at
    }

    fn free(&mut self, at: u32) {
        self.live.retain(|a| *a != at);
    }

    fn call(&mut self, at: u32, args: &[u32]) -> Ret {
        self.calls.push((at, args.to_vec()));
        let eax = match at {
            addr::FORM_BY_ID => self.forms.get(&args[0]).copied().unwrap_or(0),
            addr::QUEST_SET_STAGE => u32::from(self.set_stage(args[0], args[1] as u8)),
            addr::CONSOLE_MANAGER => self.console_manager,
            _ => 0,
        };
        Ret { eax, st0: 0.0 }
    }
}
