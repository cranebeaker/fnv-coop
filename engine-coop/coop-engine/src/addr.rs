//! Program facts: addresses and layouts of FalloutNV.exe 1.4.0.525.
//!
//! Each constant names its source:
//! - `legacy`: the project's own plugin for the original program (branch `legacy-xnvse`, MIT),
//!   its `docs/engine-addresses.md` unless a file is named;
//! - `ghidra`: our analysis of the program, by function address.

/// `LookupFormByID`: cdecl (form id), the form or 0.
// legacy: section 2 (prologue verified).
pub const FORM_BY_ID: u32 = 0x0048_39c0;

/// `TESQuest::SetStage`: method (quest, stage), true when the quest has the stage. It marks
/// the stage done, starts the quest, raises the current stage and runs the result scripts.
// legacy: section 2. ghidra: 0060d510, 0060f4a0.
pub const QUEST_SET_STAGE: u32 = 0x0060_d510;

/// A quest's flags, one byte: [`quest_flag`].
// ghidra: 0060ca30 (`TESQuest::SetCompleted` tests and sets bit 1 of the byte at +0x3c),
// 0060c9c0 (`TESQuest::SetRunning` sets and clears bit 0 of the same byte).
pub const QUEST_FLAGS: u32 = 0x3c;

/// The bits of a quest's flags byte.
pub mod quest_flag {
    /// The quest runs.
    // ghidra: 0060c9c0.
    pub const RUNNING: u8 = 1;
    /// The quest is completed.
    // ghidra: 0060ca30.
    pub const COMPLETED: u8 = 2;
    /// The quest failed. The program sets [`COMPLETED`] with it.
    // ghidra: 0060caf0 (`TESQuest::SetFailed` sets 0x40 and 2 in the byte at +0x3c).
    // legacy: section 6 (the address of `TESQuest::SetFailed`).
    pub const FAILED: u8 = 0x40;
}

/// A quest's stage list: the head node. A node is {item pointer, next node pointer}. An empty
/// list has both zero.
// ghidra: 005d43c0 (the list is at +0x44), 008256d0 (the empty test), 00726070 (next at +4),
// 0060d510 (`SetStage` walks it).
pub const QUEST_STAGES: u32 = 0x44;

/// `TESQuest`: the pointer to the quest's own script (0: the quest has none).
/// Source: measured with the probe on 5 quests (2026-11-10): the word at +0x1C is a `Script`
/// whose owner word ([`SCRIPT_QUEST`]) is the quest again. The legacy plugin's layout table
/// has the same member (`TESScriptableForm` at +0x18, its script at +4).
pub const QUEST_SCRIPT: u32 = 0x1c;

/// A stage item: the stage number, one byte.
// ghidra: 009373f0 (the byte at +0), 0060d510 (compared with the stage asked for).
pub const STAGE_NUMBER: u32 = 0;
/// A stage item: done, one byte (1 when the stage ran).
// ghidra: 0060f4a0 (`QuestStageInfo::Run` writes 1 at +1). legacy: section 2.
pub const STAGE_DONE: u32 = 1;

/// A quest's objective list: the head node, as [`QUEST_STAGES`].
// ghidra: 005d43e0 (the list is at +0x4c), 0060c8e0 (`TESQuest::GetObjective` walks it).
pub const QUEST_OBJECTIVES: u32 = 0x4c;

/// An objective's id, a 32-bit word.
// ghidra: 0060c8e0 (compared with the id asked for, read at +4). legacy: section 3.
pub const OBJECTIVE_ID: u32 = 4;
/// An objective's status, a 32-bit word: bit 0 displayed, bit 1 completed.
// legacy: section 3 (`BGSQuestObjective`, with its setter 0x5EC5D0).
pub const OBJECTIVE_STATUS: u32 = 0x20;

/// A quest's script event list (a pointer, 0 when the quest has no script variables yet).
// ghidra: 0059e300 (the quest's list is read at +0x5c for `GetScriptVariable`).
pub const QUEST_EVENT_LIST: u32 = 0x5c;
/// An event list's variables: a pointer to a list's head node ([`NODE_ITEM`], [`NODE_NEXT`]).
// ghidra: 0084e3a0 (the pointer at +0xc), 005a9140 (the walk that finds a variable by id).
pub const EVENT_LIST_VARS: u32 = 0xc;
/// A script variable's id, a 32-bit word (the variable's index in its script).
// ghidra: 005a9140 (compared with the id asked for, read at +0).
pub const VAR_ID: u32 = 0;
/// A script variable's value, a 64-bit float.
// ghidra: 005a9140 (the value is the double at +8).
pub const VAR_VALUE: u32 = 8;

/// A quest's current stage, one byte. `SetStage` only raises it.
// legacy: section 2 (the note of `TESQuest::SetStage`). ghidra: 0060d510 (byte at +0x60).
pub const QUEST_STAGE: u32 = 0x60;

/// A global's value (`TESGlobal`), one float.
// legacy: section 3 (the getter 0x526AC0 reads the float at +0x24).
pub const GLOBAL_VALUE: u32 = 0x24;

/// A list's node: the item pointer.
pub const NODE_ITEM: u32 = 0;
/// A list's node: the next node pointer.
// ghidra: 00726070.
pub const NODE_NEXT: u32 = 4;

/// The player (`PlayerCharacter**`, reference 0x14).
// legacy: plugin/avatar/engine/engine_pins.h (`kThePlayerPtr`).
pub const PLAYER: u32 = 0x011d_ea3c;
/// Where a reference keeps its position (`TESObjectREFR` posX/Y/Z, three floats).
// legacy: section 3.
pub const REF_POSITION: u32 = 0x30;

/// The console manager: cdecl (create), the manager or 0. Its first word is the context that
/// the console's scripts compile in.
// ghidra: 0071b160.
pub const CONSOLE_MANAGER: u32 = 0x0071_b160;
// How the program runs a line typed in its console (ghidra: 0071b210, the console's key
// handler, at its two calls of `Script::Run`): it makes a `Script` on the stack, marks the
// form temporary, sets its text, runs it in the manager's context, and destroys it.
/// `Script`'s constructor: method (script).
// ghidra: 005aa0f0 (sets `Script::vftable`, then two members at +0x44 and +0x4c).
pub const SCRIPT_NEW: u32 = 0x005a_a0f0;
/// Bytes for a `Script`: the constructor's last member is 8 bytes at +0x4c.
pub const SCRIPT_BYTES: u32 = 0x54;
/// Marks a form temporary (flag 0x4000 at +8): method (form).
// ghidra: 00484490.
pub const FORM_SET_TEMPORARY: u32 = 0x0048_4490;
/// Sets a script's text (a copy, kept at +0x2c): method (script, text).
// ghidra: 005abe50.
pub const SCRIPT_SET_TEXT: u32 = 0x005a_be50;
/// Compiles the script's text and executes it: method (script, context, 1, calling reference).
// legacy: section 2 (`Script::Run`). ghidra: 005ac400.
pub const SCRIPT_RUN: u32 = 0x005a_c400;
/// `Script`'s destructor (it frees the text and the compiled data, not the object): method
/// (script).
// ghidra: 005aa1a0 (the console's key handler calls it on its stack object).
pub const SCRIPT_DROP: u32 = 0x005a_a1a0;

/// A form's id, a 32-bit word (each form: a quest, a script, a global).
// legacy: plugin/avatar/engine/engine_pins.h (`kFormRefId`).
pub const FORM_ID: u32 = 0x0c;

/// The script runner that each script run passes: method (manager, script, calling reference,
/// event list, containing object, flag, three effect words). `Script::Execute` and the three
/// effect entries call it. The higher entries do not see each run.
// ghidra: 005ac1e0 (`Script::Execute` calls it with the script, its three arguments, the flag
// and three zeros), 005e2590. Runs on the real program: the nine argument words, and 26,450 of
// 26,450 runs in play, while an override of a higher entry missed effect and console runs.
pub const SCRIPT_RUNNER: u32 = 0x005e_2590;
/// The runner's argument words: the script and the calling reference.
pub const RUNNER_ARG_SCRIPT: usize = 1;
/// See [`RUNNER_ARG_SCRIPT`].
pub const RUNNER_ARG_REFERENCE: usize = 2;
/// A script's type, a 16-bit word: [`script_type`].
// legacy: section 6 (`Script::info.type`).
pub const SCRIPT_TYPE: u32 = 0x28;
/// The values of a script's type.
pub mod script_type {
    /// An object's script (a reference, an item, an actor).
    pub const OBJECT: u16 = 0;
    /// A quest's script.
    pub const QUEST: u16 = 1;
    /// A magic effect's script.
    pub const EFFECT: u16 = 0x100;
}
/// A script's owner quest (a pointer, 0 for a script that no quest owns).
// ghidra: 005ac1e0 (the pointer at +0x40 is tested and read for the quest's script delay).
pub const SCRIPT_QUEST: u32 = 0x40;

/// `TESQuest::SetRunning`: method (quest, bool). It sets or clears [`quest_flag::RUNNING`].
// legacy: section 2. ghidra: 0060c9c0.
pub const QUEST_SET_RUNNING: u32 = 0x0060_c9c0;
/// `TESQuest::SetCompleted`: method (quest, bool).
// legacy: section 2. ghidra: 0060ca30.
pub const QUEST_SET_COMPLETED: u32 = 0x0060_ca30;
/// `TESQuest::SetFailed`: method (quest, bool).
// legacy: section 6. ghidra: 0060caf0.
pub const QUEST_SET_FAILED: u32 = 0x0060_caf0;
/// `BGSQuestObjective::SetState`: method (objective, flags 0..3: bit 0 displayed, bit 1
/// completed). It shows the HUD's notices.
// legacy: section 2 (prologue verified).
pub const OBJECTIVE_SET_STATE: u32 = 0x005e_c5d0;
/// An objective's quest (a pointer).
// legacy: section 3 (`BGSQuestObjective`: `quest` +0x10).
pub const OBJECTIVE_QUEST: u32 = 0x10;
/// The player's objective list: a list ([`NODE_ITEM`], [`NODE_NEXT`]) of objectives whose
/// head node is inline in the player at this offset. The Pip-Boy's quest page walks it:
/// an objective whose status is only written is not in it. [`OBJECTIVE_SET_STATE`] adds the
/// objective to it.
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs
// (`PLAYER_OBJECTIVE_LIST_OFFSET`: 0079bc30 answers this + 0x6bc). Not re-measured here.
pub const PLAYER_OBJECTIVES: u32 = 0x6bc;
/// Adds an objective to the player's objective list when it is not in it: method (player,
/// objective). [`OBJECTIVE_SET_STATE`] calls it.
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs
// (`PLAYER_UPDATE_OBJECTIVE`, fn_00952a20). Not re-measured here.
pub const PLAYER_UPDATE_OBJECTIVE: u32 = 0x0095_2a20;

// ---- The avatar (SPEC-06). Each fact was called or read on the game program by a planner:
// ---- the run names are folders and scripts under the project's notes (m3/A, m3/B, m3/C).

/// The form id of the player's base form (`TESNPC`).
// legacy: the player's base is form 7. runs: m3/B e9, m3/C e4 (a Character from its copy).
pub const PLAYER_BASE_ID: u32 = 7;
/// Copies an NPC base form: method (form, 0, 0), the copy.
// ghidra: 004867a0 (the form's table slot +0x40). run: m3/B e9.
pub const NPC_COPY: u32 = 0x0048_67a0;
/// Sets a base form's sex: method (base + [`NPC_SEX_DATA`], 1, female, 1).
// ghidra: 005c65a0 (the `SexChange` command calls it). run: m3/B e17p.
pub const NPC_SET_SEX: u32 = 0x0047_dd50;
/// Where a base form keeps the data that [`NPC_SET_SEX`] changes.
// ghidra: 005c65a0.
pub const NPC_SEX_DATA: u32 = 0x30;
/// Sets a base form's race: method (base, race form, 0).
// ghidra: 005da810 (the `MatchRace` command calls it). run: m3/B e17p.
pub const NPC_SET_RACE: u32 = 0x0060_b240;
/// True (low byte) when an actor's base form is female: (actor).
// ghidra: 0087f4c0 (it reads the actor's base form). run: m3/B e17p.
pub const ACTOR_IS_FEMALE: u32 = 0x0087_f4c0;
/// The `GetIsRace` condition: (reference, race form, 0, address of a 64-bit float). It writes
/// 1.0 when the reference's base form has this race.
// ghidra: 0059f240. run: m3/A e6.
pub const REF_IS_RACE: u32 = 0x0059_f240;
/// Makes a reference of a base form at an anchor reference: (anchor, base, count, 0, 0, 1.0),
/// the reference.
// legacy: `Call_PlaceAtMe`. ghidra: 005c4a70 calls it. runs: m3/A e3, m3/B e9, m3/C e4.
pub const PLACE_AT_ME: u32 = 0x005c_4b30;
/// Restrains an actor, or lets it go: method (actor, on).
// ghidra: 005d0920 (the `SetRestrained` command calls it). runs: m3/A e4, m3/B e9, m3/C e6.
pub const ACTOR_RESTRAIN: u32 = 0x008a_ce50;
/// An actor's life state ([`life_state`]).
// legacy: section 3. ghidra: 004f8960. runs: m3/B e9 (5 while restrained), e10 (1 after
// lethal damage).
pub const ACTOR_LIFE_STATE: u32 = 0x108;
/// The values of [`ACTOR_LIFE_STATE`] that the adapter tests.
pub mod life_state {
    /// Alive, not held.
    pub const ALIVE: u32 = 0;
    /// Dying.
    pub const DYING: u32 = 1;
    /// Dead.
    pub const DEAD: u32 = 2;
    /// Restrained: alive, and its AI does not move it.
    pub const RESTRAINED: u32 = 5;
}
/// An actor's mover.
// legacy: section 3. ghidra: 005f36f0 (answers the word at +0x34 of actor+0x15C). run: m3/B e9.
pub const ACTOR_MOVER: u32 = 0x190;
/// Sets the mover's movement flag override: method (mover, flags).
// ghidra: 009de060 (the flags go to +0x38, and the byte at +0x71 becomes 1). runs: m3/B e4c2,
// e16, m3/C e4.
pub const MOVER_SET_FLAGS: u32 = 0x009d_e060;
/// Clears the mover's movement flag override: method (mover).
// ghidra: 009de080.
pub const MOVER_CLEAR_FLAGS: u32 = 0x009d_e080;
/// The mover's override flags.
// ghidra: 009de060.
pub const MOVER_FLAGS: u32 = 0x38;
/// The movement flags.
// legacy: section 3. ghidra: 009ddc00. run: m3/B e16 (each flag gives its animation group).
pub mod move_flag {
    /// Forward.
    pub const FORWARD: u32 = 1;
    /// Back.
    pub const BACK: u32 = 2;
    /// Left.
    pub const LEFT: u32 = 4;
    /// Right.
    pub const RIGHT: u32 = 8;
    /// Walk.
    pub const WALK: u32 = 0x100;
    /// Run.
    pub const RUN: u32 = 0x200;
    /// Sneak.
    pub const SNEAK: u32 = 0x400;
}
/// Sets one coordinate of a reference's place: (reference, axis letter, value).
// ghidra: 005c05f0 (the `SetPos` command calls it). runs: m3/A e5b, m3/B e9.
pub const REF_SET_POS: u32 = 0x005c_0740;
/// Sets one angle of a reference, in degrees: (reference, axis letter, value).
// ghidra: 005c09c0 (the `SetAngle` command calls it). run: m3/B e9.
pub const REF_SET_ANGLE: u32 = 0x005c_0b10;
/// A reference's three angles, in radians.
// Measured with the probe (2026-11-10): `SetAngle z 90` gives 1.571 in the third float.
pub const REF_ANGLE: u32 = 0x24;
/// A reference's cell.
// legacy: section 3 (`parentCell` +0x40).
pub const REF_CELL: u32 = 0x40;
/// Disables a reference: (reference, 0).
// ghidra: 005aa500 (the `Disable` command calls it). run: m3/B e17p.
pub const REF_DISABLE: u32 = 0x005a_a500;
/// The `MarkForDelete` command's handler: (0, 0, reference, 0, 0, 0, 0, 0). The game destroys
/// the reference in its next frame.
// ghidra: 005d8e20. run: m3/B e17p (a plain reference goes with no fault).
pub const REF_MARK_FOR_DELETE: u32 = 0x005d_8e20;
/// An actor's animation data: method (actor), the data or 0.
// ghidra: 008b70d0. run: m3/B e9.
pub const ACTOR_ANIM_DATA: u32 = 0x008b_70d0;
/// The animation data's group word: the movement group is its high half.
// Measured: m3/B e9 (0x00030000 for forward), e16 (0x00070000 run, 0x10031000 sneak).
pub const ANIM_GROUP: u32 = 0x4c;
/// A form's flags word. Bit 0x800: a disabled reference. Bit 0x4000: a temporary form.
// ghidra: 00484490 (the temporary mark), 00440da0 (the disabled test).
pub const FORM_FLAGS: u32 = 8;

// ---- The game's map of changed forms (SPEC-15). Each fact was read on the game program by
// ---- three planners (runs under the project's notes: ws/A, ws/B, ws/C).

/// Where the program keeps the address of its save manager.
// ghidra: 0084a690, 00484b60 (the mark method reads it). runs: ws/A e1, ws/B e5, ws/C e9.
pub const SAVE_MANAGER: u32 = 0x011d_df38;
/// The save manager: the address of its map of changed forms.
// ghidra: 008457b0. runs: ws/B e5, ws/C e9.
pub const SAVE_CHANGE_MAP: u32 = 0;
/// The change map: the count of its buckets. A form's bucket is its id modulo the count.
// ghidra: 009a4250, 008456e0. runs: ws/B e7, ws/C e9.
pub const MAP_BUCKET_COUNT: u32 = 4;
/// The change map: the address of its bucket array (one item address for each bucket).
pub const MAP_BUCKETS: u32 = 8;
/// The change map: the count of its forms.
pub const MAP_COUNT: u32 = 0xc;
/// A map item: the next item of the same bucket.
// run: ws/C e9 (a node reads `next, form id, entry`).
pub const MAP_ITEM_NEXT: u32 = 0;
/// A map item: the form's id.
pub const MAP_ITEM_FORM: u32 = 4;
/// A map item: the address of the form's change entry.
pub const MAP_ITEM_ENTRY: u32 = 8;
/// A change entry: the change flags.
// runs: ws/A e1 (`80000001` after `Disable`), ws/B e5.
pub const CHANGE_FLAGS: u32 = 0;
/// Marks a form as changed: method (map, form id, flags). The game's own code calls it.
// ghidra: 008457b0. runs: ws/A e1, ws/B e4, ws/C e8 (a script command's change reaches it).
pub const MAP_MARK: u32 = 0x0084_57b0;

// ---- One form's change record (SPEC-15). The save steps are the steps of the game's own
// ---- "save one form" (ghidra: 00849730). Runs: ws/A e3h (host), e3gp (guest), e5ga.

/// The save manager: the address of its table of form indexes.
// ghidra: 0084a690. run: ws/A e3h.
pub const SAVE_IDS: u32 = 8;
/// The form index table: the next free index.
// run: ws/A e3h, e3gp.
pub const IDS_COUNTER: u32 = 0x20;
/// The form id of an index: method (table, index).
// ghidra: 00846d20. run: ws/A e3h.
pub const IDS_FORM_OF: u32 = 0x0084_6d20;
/// Adds one pair to a table half: method (half, key, value). The first half (the table) maps
/// a form id to its index. The second half ([`IDS_REVERSE`]) maps an index to its form id.
// ghidra: 00844700. run: ws/A e3gp.
pub const IDS_INSERT: u32 = 0x0084_4700;
/// The form index table's second half.
pub const IDS_REVERSE: u32 = 0x10;
/// Where the program keeps the object that knows the save version.
// run: ws/A e3h.
pub const SAVE_VERSION_SOURCE: u32 = 0x011d_e134;
/// The save version (low byte): method (object).
// ghidra: 00851110. run: ws/A e3h.
pub const SAVE_VERSION: u32 = 0x0085_1110;
/// A form's type (low byte), as the save names it: method (form).
// ghidra: 00401170 (used by the `GetIsRace` condition for the same test). run: ws/A e3h.
pub const FORM_TYPE: u32 = 0x0040_1170;
/// The save buffer of one form: its constructor, method (buffer).
// ghidra: 008659c0 (it sets the `BGSSaveFormBuffer` table and clears +0x20).
pub const SAVE_BUFFER_NEW: u32 = 0x0086_59c0;
/// Bytes for a save buffer object.
// run: ws/A e3h (64 bytes, the constructor's last member is at +0x20).
pub const SAVE_BUFFER_BYTES: u32 = 64;
/// Starts the buffer again: method (buffer). It clears the length.
// ghidra: 00847db0.
pub const SAVE_BUFFER_RESET: u32 = 0x0084_7db0;
/// Sets the record's head: method (buffer, form id, flags, type, version).
// ghidra: 00865ad0.
pub const SAVE_BUFFER_HEADER: u32 = 0x0086_5ad0;
/// Gives the buffer its form: method (buffer, form).
// ghidra: 0050f9c0.
pub const SAVE_BUFFER_FORM: u32 = 0x0050_f9c0;
/// A form's table slot of its own save: method (form, buffer).
// run: ws/A e3h.
pub const FORM_SAVE_SLOT: u32 = 0x80;
/// A form's table slot of the save's last step: method (form, buffer).
pub const FORM_SAVE_END_SLOT: u32 = 0x54;
/// The save's middle step (the form's extra data): method (buffer, form, flags).
// ghidra: 0084eb80.
pub const SAVE_EXTRA: u32 = 0x0084_eb80;
/// The save buffer: the address of its bytes.
pub const SAVE_BUFFER_DATA: u32 = 4;
/// The save buffer: the count of its bytes.
pub const SAVE_BUFFER_LENGTH: u32 = 0xc;
/// The most bytes of one record that the adapter takes. Measured records: 19 bytes (a
/// reference) to 1400 bytes (an actor).
pub const RECORD_LIMIT: u32 = 0x1_0000;
/// The change entry of a form: method (map, form id), the entry or 0.
// ghidra: 009a4250. runs: ws/A e3h, ws/C e8.
pub const MAP_ENTRY: u32 = 0x009a_4250;
/// A change entry: the address of its waiting record (type byte, version byte, length, bytes).
// run: ws/A e3gp.
pub const ENTRY_PENDING: u32 = 4;
/// Gives an entry a waiting record: method (entry + [`ENTRY_PENDING`], bytes, length, type,
/// version). The game copies the bytes into a block of its own.
// ghidra: 008665d0 (it allocates the block and copies). run: ws/A e3gp.
pub const ENTRY_ATTACH: u32 = 0x0086_65d0;
/// Loads a form's waiting record into the form: method (save manager, form).
// ghidra: 008495d0 (the game calls it for each form of a cell that loads: ws/A e5ga, 414
// calls of its own). run: ws/A e3gp.
pub const SAVE_LOAD_FORM: u32 = 0x0084_95d0;
/// Ends a load: method (save manager, 0).
// ghidra: 008492b0. run: ws/A e3gp.
pub const SAVE_LOAD_FINISH: u32 = 0x0084_92b0;
/// Where a guest's own form indexes start after its join: the host's indexes stay below.
// run: ws/A e3gp (the counter was set to this value, and the host's record then loaded). A
// record holds an index in 3 bytes.
pub const GUEST_INDEX_BASE: u32 = 0x0040_0000;
/// Gives a block of the game's own heap back: (block).
// ghidra: 00401030 (the pair of the allocator 00401000 that [`ENTRY_ATTACH`] uses).
pub const HEAP_FREE: u32 = 0x0040_1030;

// ---- The avatar's damage rules (SPEC-06 slice 4).

/// The game's hit funnel: method (victim actor, hit data, flag). Each hit of a weapon, a
/// projectile or a melee attack goes through it.
// legacy: section 9 (`ApplyHitData`, six callers, no other reference). runs: m3/A e10a (3
// shots, 3 calls), e10b (not run: the health stays).
pub const ACTOR_HIT: u32 = 0x0089_a760;
/// The game's kill function: method (actor, killer, word).
// legacy: `engine_pins.h`. ghidra: 0089d900. run: m3/B e10 (not run: the life state stays 5
// under lethal damage).
pub const ACTOR_KILL: u32 = 0x0089_d900;
/// The form type of a placed NPC's reference, as [`FORM_TYPE`] answers it.
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs (`FORM_TYPE_ACHR`).
// Not re-measured here.
pub const FORM_TYPE_NPC_REF: u8 = 0x3b;
/// The form type of a placed creature's reference.
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs (`FORM_TYPE_ACRE`).
// Not re-measured here.
pub const FORM_TYPE_CREATURE_REF: u8 = 0x3c;
/// The change flag of an actor's life state: the game's life state setter marks it whenever the
/// state changes, so a death is in the map of changed forms.
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs
// (`CHANGE_ACTOR_LIFESTATE`, set by `Actor::SetLifeState` 008a1800). Not re-measured here.
pub const CHANGE_LIFE: u32 = 0x400;

/// The hit data: the reference that made the hit (0: none).
// legacy: `plugin/combat/combat_addrs.h` (each offset checked in the funnel's disassembly).
pub const HIT_SOURCE: u32 = 0;
/// The hit data: the body part (14: none).
pub const HIT_LOCATION: u32 = 0x10;
/// The hit data: the health damage (a float).
pub const HIT_HEALTH_DAMAGE: u32 = 0x14;
/// The hit data: the weapon's form (0: none).
pub const HIT_WEAPON: u32 = 0x30;
/// The hit data: the flags (4: a critical hit).
pub const HIT_FLAGS: u32 = 0x58;
/// The `GetHealthPercentage` condition: (reference, 0, 0, address of a 64-bit float). It
/// writes the health as a part of 1.
// ghidra: 0059c7e0. run: m3/A e10a (1.0 to 0.6 after three shots).
pub const REF_HEALTH_PART: u32 = 0x0059_c7e0;
/// A reference's 3D object in the scene (0: none).
// legacy: section 3. runs: ws/C e8, and the two-game record run (the word is 0 after a
// disable).
pub const REF_NODE: u32 = 0x64;
/// A scene object's place: three floats.
// Measured with the probe (2026-11-10): the three floats at +0xA0 of a reference's 3D object
// equal the reference's place, and they follow the console's `SetPos`.
pub const NODE_PLACE: u32 = 0xa0;

// ---- The avatar's save rule (SPEC-06 slice 5).

/// The game's save function: method (save manager, name, word, flag). Each save goes through
/// it: a quick save, an autosave at a door, a save from the menu or the console.
// ghidra: 008503b0 (all 8 call sites). run: m3/A s1 (one call for a console save).
pub const GAME_SAVE: u32 = 0x0085_03b0;
/// The game's load function: method (save manager, name, word, flag, flag).
// ghidra: 00850760. run: m3/A s2.
pub const GAME_LOAD: u32 = 0x0085_0760;

// ---- A cell (SPEC-06 slice 3).

/// A cell's load state (one byte).
// Measured with the probe (2026-11-10) on the cell of Doc Mitchell's house: 6 while the player
// is in the cell, 3 after the player left (the cell is in the game's buffer), 0 after the
// buffer is purged. The legacy plugin tested the same state for "attached".
pub const CELL_STATE: u32 = 0x26;
/// The form type of a cell, as [`FORM_TYPE`] answers it.
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs (`FORM_TYPE_CELL`,
// `TYPE_CELL` in the translations). Not re-measured here.
pub const FORM_TYPE_CELL: u8 = 0x39;
/// True (low byte) when a cell is an interior: method (cell).
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs (`CELL_IS_INTERIOR`,
// 00425fd0). Not re-measured here.
pub const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// The worldspace of an exterior cell (0 for an interior): method (cell).
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs
// (`CELL_GET_WORLD_SPACE`, 0054ddd0). Not re-measured here.
pub const CELL_WORLD_SPACE: u32 = 0x0054_ddd0;

// ---- A move of the player (the guest goes to the host). The game moves the player in its own
// ---- frame from a request, as a door and the player's `MoveTo` do.

/// Queues a move of the player: method (player, request). The game carries it out in its own
/// frame once the cell grid can take it, and frees the request.
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs
// (`REQUEST_POSITION_PLAYER`, 0093be30). Not re-measured here.
pub const PLAYER_REQUEST_MOVE: u32 = 0x0093_be30;
/// The game's own allocator: (bytes), the block. [`HEAP_FREE`] is its pair.
// ghidra: 00401030 (the pair of this allocator). Alexanger-dev/fnv-coop branch coop-next,
// crates/coop-engine/src/addr.rs (`OPERATOR_NEW`). Not re-measured here.
pub const HEAP_ALLOC: u32 = 0x0040_1000;
/// The move request's constructor: method (request), the request.
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs
// (`POSITION_REQUEST_CONSTRUCT`, 00456540). Not re-measured here.
pub const MOVE_REQUEST_NEW: u32 = 0x0045_6540;
/// The move request's size. Its words: the worldspace (an exterior), the interior cell, the
/// place (three floats), the heading in radians (the rotation's Z), and "reset the weather"
/// (a byte).
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs
// (`PositionPlayerRequest`, 0x34 bytes). Not re-measured here.
pub const MOVE_REQUEST_BYTES: u32 = 0x34;
/// See [`MOVE_REQUEST_BYTES`].
pub const MOVE_REQUEST_WORLD: u32 = 0;
/// See [`MOVE_REQUEST_BYTES`].
pub const MOVE_REQUEST_CELL: u32 = 4;
/// See [`MOVE_REQUEST_BYTES`].
pub const MOVE_REQUEST_PLACE: u32 = 8;
/// See [`MOVE_REQUEST_BYTES`].
pub const MOVE_REQUEST_HEADING: u32 = 0x1c;
/// See [`MOVE_REQUEST_BYTES`].
pub const MOVE_REQUEST_WEATHER: u32 = 0x20;
/// The player's function table slot that answers true while the player cannot be moved:
/// method (player, 0). The player's `MoveTo` does nothing then.
// Alexanger-dev/fnv-coop branch coop-next, crates/coop-engine/src/addr.rs
// (`PLAYER_CANNOT_MOVE_SLOT`). Not re-measured here.
pub const PLAYER_CANNOT_MOVE_SLOT: u32 = 0x22c;

/// The values of [`CELL_STATE`] that the adapter tests.
pub mod cell_state {
    /// The cell's references are in the scene.
    pub const ATTACHED: u8 = 6;
}
