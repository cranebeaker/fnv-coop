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
    /// The form id of the next form that the program makes at run time.
    next_id: u32,
    /// Each actor's animation data: actor address to data address.
    animations: BTreeMap<u32, u32>,
    /// Each reference's base form: reference address to base address.
    bases: BTreeMap<u32, u32>,
    /// What each form's own save writes: form address to bytes.
    form_bytes: BTreeMap<u32, Vec<u8>>,
    /// The records that wait in the change map: form id to bytes.
    pending: BTreeMap<u32, Vec<u8>>,
    /// The form index table: index to form id.
    index_forms: BTreeMap<u32, u32>,
    /// The save version that the program answers.
    save_version: u8,
    /// The forms that the program's own load ran for, in order (form addresses).
    pub loaded: Vec<u32>,
    /// The blocks that went back to the program's heap, in order.
    pub freed: Vec<u32>,
}

/// The fake's table of form functions, and its two save slots.
const FAKE_TABLE: u32 = 0x7000_0000;
const FAKE_SAVE: u32 = 0x7100_0080;
const FAKE_SAVE_END: u32 = 0x7100_0054;

/// Where the fake keeps a base form's race. The adapter does not read it.
const FAKE_RACE: u32 = 0x110;

impl FakeProgram {
    /// An empty program.
    pub fn new() -> FakeProgram {
        FakeProgram {
            next: 0x2000_0000,
            next_id: 0xff00_0800,
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

    /// Adds the player's base form and the player's reference at `place`. Returns the
    /// reference's address.
    pub fn add_player(&mut self, place: [f32; 3]) -> u32 {
        let base = self.take(0x200);
        self.set_u32(base + addr::FORM_ID, addr::PLAYER_BASE_ID);
        self.forms.insert(addr::PLAYER_BASE_ID, base);
        let player = self.take(0x200);
        self.set_u32(player + addr::FORM_ID, 0x14);
        for (i, v) in place.iter().enumerate() {
            self.set_f32(player + addr::REF_POSITION + 4 * i as u32, *v);
        }
        self.forms.insert(0x14, player);
        self.set_u32(addr::PLAYER, player);
        player
    }

    /// Gives the program its form index table and its save version. Call it after
    /// [`FakeProgram::add_change_map`].
    pub fn add_save_tables(&mut self, version: u8) {
        let manager = self.u32(addr::SAVE_MANAGER);
        let ids = self.take(0x40);
        self.set_u32(ids + addr::IDS_COUNTER, 1);
        self.set_u32(manager + addr::SAVE_IDS, ids);
        let source = self.take(0x10);
        self.set_u32(addr::SAVE_VERSION_SOURCE, source);
        self.save_version = version;
        self.set_u32(FAKE_TABLE + addr::FORM_SAVE_SLOT, FAKE_SAVE);
        self.set_u32(FAKE_TABLE + addr::FORM_SAVE_END_SLOT, FAKE_SAVE_END);
    }

    /// The program gives a form the next index of its table, as its save does when a record
    /// first names the form. Returns the index.
    pub fn add_pair(&mut self, form: u32) -> u32 {
        let ids = self.u32(self.u32(addr::SAVE_MANAGER) + addr::SAVE_IDS);
        let index = self.u32(ids + addr::IDS_COUNTER);
        self.index_forms.insert(index, form);
        self.set_u32(ids + addr::IDS_COUNTER, index + 1);
        index
    }

    /// The form id that the table has for an index (0: none).
    pub fn form_of_index(&self, index: u32) -> u32 {
        self.index_forms.get(&index).copied().unwrap_or(0)
    }

    /// What the form's own save writes into a record.
    pub fn set_form_bytes(&mut self, form: u32, bytes: &[u8]) {
        self.form_bytes.insert(form, bytes.to_vec());
    }

    /// The record that waits in the change map for a form.
    pub fn pending_bytes(&self, form: u32) -> Option<Vec<u8>> {
        self.pending.get(&form).cloned()
    }

    /// The form id of the change entry at `entry`.
    fn form_of_entry(&self, entry: u32) -> u32 {
        let map = self.u32(self.u32(addr::SAVE_MANAGER) + addr::SAVE_CHANGE_MAP);
        let buckets = self.u32(map + addr::MAP_BUCKET_COUNT);
        let array = self.u32(map + addr::MAP_BUCKETS);
        for bucket in 0..buckets {
            let mut item = self.u32(array + 4 * bucket);
            while item != 0 {
                if self.u32(item + addr::MAP_ITEM_ENTRY) == entry {
                    return self.u32(item + addr::MAP_ITEM_FORM);
                }
                item = self.u32(item + addr::MAP_ITEM_NEXT);
            }
        }
        0
    }

    /// Adds a plain form with this id (a race, a base object, a reference with no actor
    /// parts). Returns its address.
    pub fn add_form(&mut self, id: u32) -> u32 {
        // Room for a reference's words that the adapter reads (the 3D word is at +0x64).
        let at = self.take(0x80);
        self.set_u32(at, FAKE_TABLE);
        self.set_u32(at + addr::FORM_ID, id);
        self.forms.insert(id, at);
        at
    }

    /// Gives the program its save manager and an empty change map with this many buckets.
    pub fn add_change_map(&mut self, buckets: u32) {
        let manager = self.take(0x40);
        let map = self.take(0x10);
        let array = self.take(4 * buckets);
        self.set_u32(map + addr::MAP_BUCKET_COUNT, buckets);
        self.set_u32(map + addr::MAP_BUCKETS, array);
        self.set_u32(manager + addr::SAVE_CHANGE_MAP, map);
        self.set_u32(addr::SAVE_MANAGER, manager);
    }

    /// The item of a form in the change map, or 0.
    fn change_item(&self, form: u32) -> u32 {
        let map = self.u32(self.u32(addr::SAVE_MANAGER) + addr::SAVE_CHANGE_MAP);
        let buckets = self.u32(map + addr::MAP_BUCKET_COUNT);
        let array = self.u32(map + addr::MAP_BUCKETS);
        let mut item = self.u32(array + 4 * (form % buckets));
        while item != 0 && self.u32(item + addr::MAP_ITEM_FORM) != form {
            item = self.u32(item + addr::MAP_ITEM_NEXT);
        }
        item
    }

    /// The program unloads a form: its own bytes wait in the form's change entry. Returns
    /// the block's address.
    pub fn unload_form(&mut self, form: u32, bytes: &[u8]) -> u32 {
        let entry = self.u32(self.change_item(form) + addr::MAP_ITEM_ENTRY);
        let block = self.take(bytes.len() as u32 + 8);
        self.set_u32(entry + addr::ENTRY_PENDING, block);
        self.pending.insert(form, bytes.to_vec());
        block
    }

    /// The program's own mark of a changed form: a new entry, or more flag bits on an old
    /// one. A new item goes to the head of its bucket. As the program does, it changes no
    /// flag of an entry that has a waiting record.
    pub fn mark_changed(&mut self, form: u32, flags: u32) {
        let map = self.u32(self.u32(addr::SAVE_MANAGER) + addr::SAVE_CHANGE_MAP);
        let old = self.change_item(form);
        if old != 0 {
            let entry = self.u32(old + addr::MAP_ITEM_ENTRY);
            if self.u32(entry + addr::ENTRY_PENDING) != 0 {
                return;
            }
        }
        let item = match self.change_item(form) {
            0 => {
                let buckets = self.u32(map + addr::MAP_BUCKET_COUNT);
                let slot = self.u32(map + addr::MAP_BUCKETS) + 4 * (form % buckets);
                let item = self.take(12);
                let entry = self.take(8);
                self.set_u32(item + addr::MAP_ITEM_NEXT, self.u32(slot));
                self.set_u32(item + addr::MAP_ITEM_FORM, form);
                self.set_u32(item + addr::MAP_ITEM_ENTRY, entry);
                self.set_u32(slot, item);
                let count = self.u32(map + addr::MAP_COUNT) + 1;
                self.set_u32(map + addr::MAP_COUNT, count);
                item
            }
            item => item,
        };
        let entry = self.u32(item + addr::MAP_ITEM_ENTRY);
        let all = self.u32(entry + addr::CHANGE_FLAGS) | flags;
        self.set_u32(entry + addr::CHANGE_FLAGS, all);
    }

    /// Breaks the map: the form's item points at itself. Only a test of the walk's limit
    /// uses it.
    pub fn loop_change_chain(&mut self, form: u32) {
        let item = self.change_item(form);
        self.set_u32(item + addr::MAP_ITEM_NEXT, item);
    }

    /// Adds a cell with this form id and load state. Returns its address.
    pub fn add_cell(&mut self, id: u32, state: u8) -> u32 {
        let at = self.take(0x100);
        self.set_u32(at + addr::FORM_ID, id);
        self.set_u8(at + addr::CELL_STATE, state);
        self.forms.insert(id, at);
        at
    }

    /// True when the form at `form` has the temporary mark.
    pub fn is_temporary(&self, form: u32) -> bool {
        self.u32(form + addr::FORM_FLAGS) & 0x4000 != 0
    }

    /// True when the base form at `base` is female.
    pub fn is_female(&self, base: u32) -> bool {
        self.u32(base + addr::NPC_SEX_DATA) & 1 != 0
    }

    /// A new form of the program's own making: an object of `len` bytes with the next run-time
    /// form id.
    fn make_form(&mut self, len: u32) -> u32 {
        let at = self.take(len);
        let id = self.next_id;
        self.next_id += 1;
        self.set_u32(at + addr::FORM_ID, id);
        self.forms.insert(id, at);
        at
    }

    /// The program's PlaceAtMe: an actor at the anchor's place, with a mover and animation
    /// data, alive and loose.
    fn place_at_me(&mut self, anchor: u32, base: u32) -> u32 {
        let actor = self.make_form(0x200);
        self.bases.insert(actor, base);
        for o in [0, 4, 8] {
            let v = self.f32(anchor + addr::REF_POSITION + o);
            self.set_f32(actor + addr::REF_POSITION + o, v);
        }
        let cell = self.u32(anchor + addr::REF_CELL);
        self.set_u32(actor + addr::REF_CELL, cell);
        let mover = self.take(0x80);
        self.set_u32(actor + addr::ACTOR_MOVER, mover);
        let animation = self.take(0x60);
        self.set_u32(animation + addr::ANIM_GROUP, 0xffff_0000);
        self.animations.insert(actor, animation);
        actor
    }

    /// The program's own choice of a movement group for a mover's flags, as measured: forward
    /// walks (3) or runs (7), and a sneak has 0x1000 more. No direction: no group.
    fn set_mover_flags(&mut self, mover: u32, flags: u32) {
        self.set_u32(mover + addr::MOVER_FLAGS, flags);
        self.set_u8(mover + 0x71, u8::from(flags != 0));
        let group = match flags {
            0 => 0xffff,
            f => {
                let gait = if f & addr::move_flag::RUN != 0 { 7 } else { 3 };
                let sneak = if f & addr::move_flag::SNEAK != 0 {
                    0x1000
                } else {
                    0
                };
                gait | sneak
            }
        };
        let owner = self
            .animations
            .iter()
            .find(|(actor, _)| self.u32(**actor + addr::ACTOR_MOVER) == mover)
            .map(|(_, data)| *data);
        if let Some(data) = owner {
            self.set_u32(data + addr::ANIM_GROUP, group << 16);
        }
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
            addr::NPC_COPY => self.make_form(0x200),
            addr::NPC_SET_SEX => {
                self.set_u32(args[0], args[2] & 1);
                0
            }
            addr::FORM_SET_TEMPORARY => {
                let flags = self.u32(args[0] + addr::FORM_FLAGS) | 0x4000;
                self.set_u32(args[0] + addr::FORM_FLAGS, flags);
                0
            }
            addr::PLACE_AT_ME => self.place_at_me(args[0], args[1]),
            addr::MAP_MARK => {
                self.mark_changed(args[1], args[2]);
                0
            }
            addr::MAP_ENTRY => match self.change_item(args[1]) {
                0 => 0,
                item => self.u32(item + addr::MAP_ITEM_ENTRY),
            },
            addr::SAVE_VERSION => u32::from(self.save_version),
            // The fake has one type for each form: a reference.
            addr::FORM_TYPE => 0x3a,
            addr::IDS_FORM_OF => self.form_of_index(args[1]),
            addr::IDS_INSERT => {
                let ids = self.u32(self.u32(addr::SAVE_MANAGER) + addr::SAVE_IDS);
                if args[0] == ids + addr::IDS_REVERSE {
                    self.index_forms.insert(args[1], args[2]);
                }
                0
            }
            addr::SAVE_BUFFER_RESET => {
                self.set_u32(args[0] + addr::SAVE_BUFFER_LENGTH, 0);
                0
            }
            // The form's own save: its bytes go into the buffer.
            FAKE_SAVE => {
                let bytes = self.form_bytes.get(&args[0]).cloned().unwrap_or_default();
                let data = self.take(bytes.len() as u32);
                self.write(data, &bytes);
                self.set_u32(args[1] + addr::SAVE_BUFFER_DATA, data);
                self.set_u32(args[1] + addr::SAVE_BUFFER_LENGTH, bytes.len() as u32);
                0
            }
            addr::ENTRY_ATTACH => {
                let form = self.form_of_entry(args[0] - addr::ENTRY_PENDING);
                let mut bytes = vec![0; args[2] as usize];
                self.read(args[1], &mut bytes);
                // The program copies the bytes into a block of its own heap.
                let block = self.take(args[2] + 8);
                self.set_u32(args[0], block);
                self.pending.insert(form, bytes);
                0
            }
            addr::HEAP_FREE => {
                self.freed.push(args[0]);
                0
            }
            addr::SAVE_LOAD_FORM => {
                self.loaded.push(args[1]);
                0
            }
            addr::NPC_SET_RACE => {
                self.set_u32(args[0] + FAKE_RACE, args[1]);
                0
            }
            addr::ACTOR_IS_FEMALE => {
                let base = self.bases.get(&args[0]).copied().unwrap_or(0);
                u32::from(base != 0 && self.is_female(base))
            }
            addr::REF_IS_RACE => {
                let base = self.bases.get(&args[0]).copied().unwrap_or(0);
                let same = base != 0 && self.u32(base + FAKE_RACE) == args[1];
                self.write(args[3], &f64::from(u8::from(same)).to_le_bytes());
                1
            }
            addr::ACTOR_RESTRAIN => {
                let state = if args[1] != 0 {
                    addr::life_state::RESTRAINED
                } else {
                    addr::life_state::ALIVE
                };
                self.set_u32(args[0] + addr::ACTOR_LIFE_STATE, state);
                0
            }
            addr::REF_SET_POS => {
                let axis = args[1].saturating_sub(u32::from(b'X')).min(2);
                self.set_u32(args[0] + addr::REF_POSITION + 4 * axis, args[2]);
                0
            }
            addr::REF_SET_ANGLE => {
                let axis = args[1].saturating_sub(u32::from(b'X')).min(2);
                let radians = f32::from_bits(args[2]).to_radians();
                self.set_f32(args[0] + addr::REF_ANGLE + 4 * axis, radians);
                0
            }
            addr::MOVER_SET_FLAGS => {
                self.set_mover_flags(args[0], args[1]);
                0
            }
            addr::MOVER_CLEAR_FLAGS => {
                self.set_mover_flags(args[0], 0);
                0
            }
            addr::ACTOR_ANIM_DATA => self.animations.get(&args[0]).copied().unwrap_or(0),
            addr::REF_DISABLE => {
                let flags = self.u32(args[0] + addr::FORM_FLAGS) | 0x800;
                self.set_u32(args[0] + addr::FORM_FLAGS, flags);
                0
            }
            // The program destroys the reference: its id answers nothing after.
            addr::REF_MARK_FOR_DELETE => {
                self.forms.retain(|_, at| *at != args[2]);
                0
            }
            _ => 0,
        };
        Ret { eax, st0: 0.0 }
    }
}
