# Engine addresses: FalloutNV.exe 1.4.0.525 (Steam)

Lane L5. Owner of this file: L5. Companion code: `plugin/hooks/` (`engine_addrs.h` is GENERATED from the table data by
`plugin/hooks/gen_addrs.py`; the prologue bytes below are what the runtime patcher compares before it writes anything).


## 1. Status of every address listed in docs/02-engine-re.md

| 02-engine-re item | addr | verdict | evidence |
|---|---|---|---|
| `TESQuest::SetStage` | 0x60D510 | **V** | `55 8B EC 83 EC 08 89 4D F8 8B 4D F8 E8 9F 6E FC FF`; `Cmd_SetStage` (0x5C7140) calls it; body: stage lookup, `currentStage(+0x60)` store, `QuestStageInfo::Run`, `vtbl+0x48(0x80000000)`; `ret 4` |
| JIP SetQuestStage hook | 0x60F4A9 | **V (site)**, not used | instruction boundary inside `QuestStageInfo::Run` (0x60F4A0), 0x60F4C6 is `movzx ecx,al` exactly as JIP's jump target; `[this+1]=isDone` write at 0x60F4D0. We do NOT use mid-function hooks: SetStage entry + `QuestStageItem::RunResult` give the same information with prologue-checkable entries |
| `TESTopicInfo::RunResultScript` | 0x61F170 | **V** | `55 8B EC 83 EC 0C 89 4D F4 83 7D 0C 00 75 05 E9`; returns early if actor==0; calls `GetResultScript` 0x61EB60; `ret 8`; 14 callers |
| RunResultScript hook 0x61F184 | 0x61F184 | **V (site)**, not used | boundary right after the `jne` of the actor check |
| `GetResultScript` | 0x61EB60 | **V** | SEH-frame function, called by RunResultScript |
| `GetTopicInfo` | 0x61A7D0 | **M** | entry/prologue ok, single caller 0x61A790; argument semantics unchecked |
| `Script::Execute` | 0x5AC1E0 | **V** | thiscall, 4 stack args, `ret 0x10`; 33 callers incl. `QuestStageItem::RunResult` and `RunResultScript` |
| `Script::Run` | 0x5AC400 | **M** | actually "compile (0x5AEDB0) + Execute"; callers are inside the console line handler 0x71B210 |
| `ConsoleManager::Print` (was "Console_Print") | 0x71D0A0 | **V** | thiscall(this, fmt, va_list), ret 8; 5 callers; not hooked |
| cell change hook | 0x94CB35 | **M (site)** | boundary inside 0x94CA50: `cmp [ebx+8],eax; jne 0x94CB35` -> `call 0x94DBA0`; not hooked by L5 |
| fast travel hook | 0x93BF22 | **M (site)** | boundary inside 0x93BEA0: `mov ecx,[ThePlayer]; call 0x93CDF0` |
| DamageActorValue hook | 0x66EE72 | **M (site)** | boundary inside 0x66EE50 (AV-change dispatcher: `if (info->cb(+0x54)) cb(...)`) |
| `Actor::GetActorValue` | 0x66EF50 | **M** | entry ok; its `this` is the ActorValueOwner (calls vtbl+0xC) |
| vtbl TESQuest 0x104AC44, TESTopicInfo 0x104D5B4, TESTopic 0x104D19C, TESReputation 0x104BA24 | | **V** | slots 0..3 are the shared TESForm functions (0x483710, 0x483710, 0x485340, 0x4853E0), slots 4+ point into `.text` |
| vtbl BGSQuestObjective 0x1047088 | | **V** | slot 0 = 0x5EC110 (destructor), single virtual |
| vtbl DialogMenu 0x107257C | | **V** | slots point into `.text` |
| vtbl ExtraSayToTopicInfo 0x1015F48, ExtraSayTopicInfoOnceADay 0x1015F3C | | **U** | slots do not look like a vtable (data after slot 1); probably wrong offsets or not vtables |
| DialogMenu singleton 0x11D9510 | | **U** | not checked (null before the menu is created) |
| ThePlayer 0x11DEA3C | | **V** | written once at 0x86D268 (player creation), 4,489 code xrefs; `Cmd_*` use it |
| console-open flag 0x11DEA2E | | **U** | 7 xrefs, not examined |
| ToggleConsole hook 0x70E5FE | | **S** | bytes are `E8 5D 60 31 00` (a call) as expected |
| LocationDiscover 0x77955D, StartCombat 0x9001C6, SetCombatTarget 0x986C79, AddNote 0x966AF9, Activate hook 0x573512, MarkScriptEvent 0x5AC763, ScriptRunner 0x5E0EC9/0x5E1716 | | **S** | bytes at each address decode as sane call/push sequences; semantics not re-derived (not needed by L5) |
| `TESObjectREFR::Activate` | 0x573170 | **S** | entry/prologue ok |
| Condition/ConditionList::Evaluate 0x681600/0x680C60 | | **S** | not checked |
| TESQuest layout (flags 0x3C, stages 0x44, currentStage 0x60) | | **V (0x60)** | SetStage reads/writes byte at +0x60 and orders by it |
| TESReputation layout (max 0x40, pos 0x44, neg 0x48) | | **V** | `Set` 0x615580 writes +0x44 when type==1 else +0x48; `Add` clamps against +0x40 |
| GMSTs sQuestUpdatedText 0x11D4504 / sQuestAddedText 0x11D282C | | **U** | only referenced from their constructors/atexit and a data slot +12; HUD notices are issued inside `BGSQuestObjective::SetState` via 0x77A5B0/0x77A480 |

## 2. Function table (everything with a verified prologue)

Prologue = first 16 bytes of the function in the running process image. A hook is installed only if all 16
match. Note that bytes 12-15 sometimes contain a call displacement: that is intentional (it makes the signature unique
across builds) but means another mod patching the same function entry makes our install refuse (reported, never forced).

| addr | name | verified | stolen | prologue (16 bytes) | notes |
|---|---|---|---|---|---|
| `0x4839C0` | LookupFormByID | V | 11 | `558bec51c745fc00000000833dc0541c` | cdecl (refID) -> TESForm*; uses form map at *0x11C54C0. NEW. |
| `0x526AC0` | TESGlobal::GetValue | V | 7 | `558bec51894dfc8b45fcd940248be55d` | trivial: fld [this+0x24]. TESGlobal form type is 6; float at +0x24. NEW. |
| `0x565090` | TESObjectREFR::IsDisabled | M | 6 | `558bec83ec0c894df468920000008b4d` | thiscall(ref)->bool: ExtraData 0x92 flags & 8. NEW. |
| `0x5650D0` | TESObjectREFR::SetDisabled | M | 5 | `558bec6aff683bb2f00064a100000000` | thiscall(ref, bool), ret 4. Creates ExtraFlags 0x92 as needed. Cmd_Enable passes 0, Cmd_Disable 1. NEW. |
| `0x573170` | TESObjectREFR::Activate | S | 9 | `558bec81ec38010000a1bc161c0133c5` | prologue ok; semantics from NVSE |
| `0x59E400` | TESQuest::IsX_59E400 | M | 7 | `558bec51894dfc8b4dfce8719c00000f` | bool test on quest used by SetStage / SetObjective*; probably IsCompleted. NEW. |
| `0x5AC1E0` | Script::Execute | V | 6 | `558bec83ec08894df80fb6451485c075` | thiscall(script, thisObj, eventList, containingObj, bool), ret 0x10. 33 callers incl. RunResult + RunResultScript. Gate hook point. |
| `0x5AC400` | Script::Run | M | 5 | `558bec6aff68989bef0064a100000000` | NVSE name; actually compiles (0x5AEDB0) then calls Execute; callers are inside the console line handler 0x71B210 |
| `0x5ACCB0` | ExtractArgs | V | 9 | `558bec81ec54010000837d0800750732` | cdecl varargs (paramInfo, scriptData, opcodeOffsetPtr, thisObj, containingObj, script, eventList, ...outputs). Matches NVSE. |
| `0x5B4BE0` | Cmd_AddItem | V | 5 | `558bec6aff68b88eef0064a100000000` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5B4E90` | Cmd_RemoveItem | V | 5 | `558bec6aff68f08eef0064a100000000` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5BDCD0` | Cmd_SetActorValue | V | 6 | `558bec83ec0cc745f800000000c745fc` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5BDE40` | Cmd_ModActorValue | V | 6 | `558bec83ec0cc745f800000000c745fc` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5BDF20` | Cmd_DamageActorValue | V | 6 | `558bec83ec10c745f800000000d9eed9` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5BE190` | Cmd_ForceActorValue | V | 6 | `558bec83ec18c745f800000000c745fc` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5BE2A0` | Cmd_KillActor | V | 6 | `558bec83ec2cc745f400000000c745fc` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5BE5E0` | Cmd_ResurrectActor | V | 6 | `558bec83ec0c6a0068d446180168cc41` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5C43D0` | Cmd_Enable | V | 6 | `558bec83ec18c745fc010000008d45fc` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5C45E0` | Cmd_Disable | V | 6 | `558bec83ec18c745fc000000008d45fc` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5C7140` | Cmd_SetStage | V | 6 | `558bec83ec088b4520d9eedd18c745fc` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5C71C0` | Cmd_StartQuest | V | 11 | `558bec51c745fc000000008d45fc508b` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5C7220` | Cmd_StopQuest | V | 11 | `558bec51c745fc000000008d45fc508b` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5C7280` | Cmd_CompleteQuest | V | 11 | `558bec51c745fc000000008d45fc508b` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5CCF50` | Cmd_MoveToMarker | V | 6 | `558bec83ec10c745fc00000000d9eed9` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5D5140` | Cmd_RewardXP | V | 11 | `558bec51c745fc000000008d45fc508b` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5D51A0` | Cmd_RewardKarma | V | 11 | `558bec51c745fc000000008d45fc508b` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5D7C20` | Cmd_SetObjectiveCompleted | V | 6 | `558bec83ec108d45f4508d4df8518d55` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5D7D30` | Cmd_SetObjectiveDisplayed | V | 6 | `558bec83ec108d45f4508d4df8518d55` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5DDD10` | Cmd_AddReputation | V | 6 | `558bec83ec0cc745f400000000c745f8` | Script command handler (standard cmd signature). From command table. Hooked for effect capture only where no primitive exists. |
| `0x5EC5D0` | BGSQuestObjective::SetState | V | 6 | `558bec83ec08894df88b4df8e8cf27e6` | thiscall(obj, flags 0..3), ret 4: bit0 displayed, bit1 completed. Does HUD notices (0x77A5B0/0x77A480) and may SetRunning/SetCompleted on the quest. Called by Cmd_SetObjectiveDisplayed/Completed. NEW. |
| `0x60C8E0` | TESQuest::GetObjective | V | 6 | `558bec83ec10894df08b4df0e8ef7afc` | thiscall(quest, id) -> BGSQuestObjective* (0 if missing). NEW. |
| `0x60C9C0` | TESQuest::SetRunning | V | 6 | `558bec83ec10894dfc0fb6450885c074` | thiscall(quest, bool), ret 4: flags bit0. Called by Cmd_StartQuest(1)/StopQuest(0) and by SetStage. NEW. |
| `0x60CA30` | TESQuest::SetCompleted | V | 7 | `558bec51894dfc0fb6450885c074438b` | thiscall(quest, bool), ret 4: flags bit1. Cmd_CompleteQuest. No Effect type for it yet. NEW. |
| `0x60D510` | TESQuest::SetStage | V | 6 | `558bec83ec08894df88b4df8e89f6efc` | thiscall(quest, u8 stage)->bool, ret 4. Finds stage item, auto SetRunning(true), writes currentStage(+0x60) if higher, runs QuestStageInfo::Run, then vtbl+0x48(0x80000000)=MarkModified. Cmd_SetStage calls it. |
| `0x60F4A0` | QuestStageInfo::Run | V | 6 | `558bec83ec0c894df48b4df4e8afd4fc` | thiscall(info, quest?, bool), ret 8. Sets info.isDone(+1)=1, iterates log entries -> RunResult. JIP hook 0x60F4A9 lies inside. NEW. |
| `0x60FB60` | QuestStageItem::RunResult | V | 5 | `558bec6aff68fbb0ee0064a100000000` | thiscall(item, bool), ret 4. Conditions, quest completed/failed flags, journal text, then Script::Execute. Called per log entry from QuestStageInfo::Run. NEW. |
| `0x615580` | TESReputation::Set | V | 7 | `558bec51894dfc837d0801750b8b45fc` | thiscall(rep, type, float), ret 8. Writes +0x44/+0x48 then MarkModified(2). Cmd_SetReputation. NEW. |
| `0x615730` | TESReputation::AddExact | M | 9 | `558bec81ec18010000a1bc161c0133c5` | Cmd_AddReputationExact target, thiscall(rep,type,float), ret 8, clamps to +0x40. NEW. |
| `0x615A00` | TESReputation::RemoveExact | M | 9 | `558bec81ec18010000a1bc161c0133c5` | Cmd_RemoveReputationExact target. NEW. |
| `0x615C90` | TESReputation::Add | V | 9 | `558bec81ec30010000a1bc161c0133c5` | thiscall(rep, type, level 1..5), ret 8. type 1 -> +0x44 (fame), else +0x48 (infamy); bump size from fReputationBump* GMSTs. Cmd_AddReputation. NEW. |
| `0x615FA0` | TESReputation::Remove | M | 9 | `558bec81ec30010000a1bc161c0133c5` | Cmd_RemoveReputation target, thiscall(rep,type,level), ret 8. NEW. |
| `0x61A7D0` | GetTopicInfo | M | 5 | `558bec6aff6870b8ef0064a100000000` | entry verified (SEH frame, 1 caller 0x61A790); argument semantics not checked |
| `0x61EB60` | TESTopicInfo::GetResultScript | V | 5 | `558bec6aff6822bbef0064a100000000` | thiscall(info, onEnd) -> Script* |
| `0x61F170` | TESTopicInfo::RunResultScript | V | 6 | `558bec83ec0c894df4837d0c007505e9` | thiscall(info, onEnd, actor), ret 8; returns early when actor==0; GetResultScript(onEnd) then Script::Execute. 14 callers. |
| `0x66EE50` | AVChangeDispatch | M | 7 | `558bec518b450c50e8c3faffff83c404` | cdecl-ish (this?, avCode, ...): looks up AV info, calls info->callback(+0x54). JIP DamageActorValue hook 0x66EE72 is inside |
| `0x66EF50` | Actor::GetActorValue | M | 6 | `558bec83ec08894df88b4508508b4df8` | called with ecx = avOwner (object whose vtbl[3] is GetActorValue); i.e. actor+0xA4 for Actor/Player |
| `0x71B210` | ConsoleRunLine | M | 5 | `558bec6aff684c84f00064a100000000` | thiscall(consoleMgr, const char* line) -> console command execution (reads this+0x38>0, parses the line, calls Script::Run at 0x71C3E8/0x71C846). Best candidate for exec{cmd}. NEW; test in-game. |
| `0x71D0A0` | Console_Print | V | 5 | `558bec6aff689384f00064a100000000` | CORRECTED: ConsoleManager::Print, thiscall(this, const char* fmt, va_list), ret 8 (0x840-byte buffer); 5 callers. A hook that treated it as cdecl varargs crashed the game; no longer hooked (L1 uses the bEchoConsole byte at 0x11F158C) |
| `0x94FD30` | Player::RewardKarma | V | 5 | `558bec6aff682081f10064a100000000` | thiscall(player, int), ret 4. Reads AV 0x17 (Karma) via avOwner(+0xA4) vtbl, clamps to GMST karma min/max. Cmd_RewardKarma. NEW. |

## 3. Data addresses and layouts

| addr | name | verified | notes |
|---|---|---|---|
| `0x11DEA3C` | `PlayerCharacter** g_thePlayer` | V | refID 0x14 |
| `0x11C3F2C` | `DataHandler** ` | M (612 xrefs) | `modList.modInfoList` index == load-order index (refID >> 24); `globalList` at +0xE8 (xNVSE layout) |
| `0x11C54C0` | form map (`NiTPointerMap<TESForm>*`) | V | used by `LookupFormByID` 0x4839C0 |
| `0x11C16BC` | `/GS` security cookie | V | not interesting; explains the `xor eax,ebp` in prologues |
| `0x118E8E0`..`0x1196D38` | script command table | V | 845 entries, stride 0x28: `longName, shortName, opcode, help, (needsParent:u8,pad,numParams:u16), params, execute, parse, eval, flags`. Dumped to `plugin/hooks/tools/re/command_table.json` |
| TESGlobal | | V | form type 6; `float data` at +0x24 (getter 0x526AC0 is `fld [this+0x24]`); xNVSE `sizeof==0x28` |
| BGSQuestObjective | | V | `objectiveId` +4, `quest` +0x10, `status` +0x20 (bit0 displayed, bit1 completed); `SetState` argument has the same encoding |
| TESObjectREFR | | S | `posX/Y/Z` +0x30, `parentCell` +0x40 (xNVSE); virtual `AddItem`/`RemoveItem` |
| PlayerCharacter avOwner | | M | `player+0xA4`; `RewardKarma` calls `vtbl[2]/[3]` on it with AV code **0x17 = 23 = Karma** (matches xNVSE `eActorVal_Karma`) |
| Actor vtable offsets (bytes) | | M | from the AV script commands: `+0x398` SetActorValue(av, float), `+0x3A4` ForceActorValue, `+0x3A8` ModActorValue(av, float, 0), `+0x3AC` DamageActorValue; `+0x324` Resurrect(bool,bool,bool); Player `+0x488` RewardXP(int) |

## 4. Missing-function search results (task 2)

| wanted | found | how |
|---|---|---|
| objective display/complete | `BGSQuestObjective::SetState` 0x5EC5D0 (flags 0..3), lookup `TESQuest::GetObjective` 0x60C8E0 | `Cmd_SetObjectiveDisplayed` 0x5D7D30 / `Completed` 0x5D7C20 call exactly these. Strong match to `Effect::SetObjective{flags bit0/bit1}` |
| quest start/stop | `TESQuest::SetRunning(bool)` 0x60C9C0 | `Cmd_StartQuest` 0x5C71C0 passes 1, `Cmd_StopQuest` 0x5C7220 passes 0; SetStage also calls it |
| quest complete | `TESQuest::SetCompleted(bool)` 0x60CA30 | `Cmd_CompleteQuest` 0x5C7280; also called from `QuestStageItem::RunResult`. No Effect type -> proposal |
| reputation | `TESReputation::Add/Set/Remove/AddExact/RemoveExact` 0x615C90/0x615580/0x615FA0/0x615730/0x615A00 | `Cmd_AddReputation` 0x5DDD10, `SetReputation` 0x5DDF10, `RemoveReputation` 0x5DDE10, `AddReputationExact` 0x5DDD90, `RemoveReputationExact` 0x5DDE90 |
| karma | `Player::RewardKarma(int)` 0x94FD30 + AV 23 via the AV owner | `Cmd_RewardKarma` 0x5D51A0; `ModActorValue Karma` goes through the generic AV virtuals |
| global set | **no single setter**. `set X to y` is executed inline by the script interpreter | captured by DIFFING every TESGlobal (`DataHandler+0xE8` list, `+0x24` float) around the outermost capture scope (implemented, `globalsBegin/End`). Applying = write `+0x24` |
| ref enable/disable | `TESObjectREFR::SetDisabled(bool)` 0x5650D0, `IsDisabled` 0x565090 | `Cmd_Enable` 0x5C43D0 / `Cmd_Disable` 0x5C45E0 |
| AddItem/RemoveItem | script commands `Cmd_AddItem` 0x5B4BE0 / `Cmd_RemoveItem` 0x5B4E90 (hooked at command level); apply via xNVSE's `TESObjectREFR::AddItem/RemoveItem` virtuals | the commands end in container-changes internals (0x487F70, 0x4818E0) that I did not name; command-level capture is exact enough (form + count) |
| console command execution | candidate `ConsoleRunLine` 0x71B210 (thiscall(consoleMgr, const char*)) | contains the only two callers of `Script::Run`; M confidence, needs an in-game call to confirm |
| actor kill / resurrect | `Cmd_KillActor` 0x5BE2A0 (large), `Cmd_ResurrectActor` 0x5BE5E0 -> Actor vtbl+0x324 | captured at command level (actor = thisObj) |
| move | `Cmd_MoveToMarker` 0x5CCF50 captured at command level; plain `MoveTo` is not in the command table under that name (the table has `MoveToMarker`, `MoveToMarkerWithFade`, `MoveToQuestTarget`) | position read back after the call |
| XP | `Cmd_RewardXP` 0x5D5140 -> Player vtbl+0x488 | pseudo AV code 0x10000 (proposal) |
| GetStage | `Cmd_GetStage` 0x5C7070 (read side) | no need to hook |

### Chain that matters for effect capture
`Cmd_SetStage` -> `TESQuest::SetStage` 0x60D510 -> `QuestStageInfo::Run` 0x60F4A0 -> per log entry
`QuestStageItem::RunResult` 0x60FB60 (completion flags 0x60CA30, journal text, then `Script::Execute` 0x5AC1E0).
Topic result: `TESTopicInfo::RunResultScript` 0x61F170 -> `Script::Execute`. The result script then calls the
`Cmd_*` handlers above, which reach the primitives we hook. `SetStage` is recorded on ENTRY; scopes nest and the outermost
one emits the txn.

## 5. Proposals to the shared contract (L4)
1. `CompleteQuest{FormRef quest}` / `FailQuest`: `TESQuest::SetCompleted` has no Effect type today (counted in `unmapped().questCompleted`).
2. XP: `ModActorValue` with a pseudo AV code (0x10000 = XP) is used in the meantime; better a dedicated `RewardXP{int}`.
3. `AddItem`/`RemoveItem` to non-player containers (counted in `unmapped().itemOnNonPlayer`) need a `FormRef target`.
4. `MoveRef` needs a destination `FormRef` marker (we capture the final xyz/cell after the move).

## 6. Added in round 3/4 (all prologue-verified; hooked block-only on guests unless stated)
| addr | name | notes |
|---|---|---|
| `0x5C7E60` / `0x5C7F10` | Cmd_SetFactionRank / Cmd_ModFactionRank | no Effect type yet |
| `0x5D5390` / `0x5D5420` | Cmd_AddToFaction / Cmd_RemoveFromFaction | |
| `0x5D4D90` / `0x5D4F00` | Cmd_AddPerk / Cmd_RemovePerk | |
| `0x5CC4F0` / `0x5CC7C0` | Cmd_AddScriptPackage / Cmd_RemoveScriptPackage | host-only AI |
| `0x5C1750` / `0x5C1A50` | Cmd_StartCombat / Cmd_StopCombat | left to NV:MP actor sync |
| `0x5CBF80` / `0x5CC120` | Cmd_Lock / Cmd_UnLock | |
| `0x5B59F0` | Cmd_Activate (script-initiated Activate) | |
| `0x60CAF0` | TESQuest::SetFailed(bool), ret 4 | FailQuest; no Effect type, counted in `unmapped().questFailed` |
| `0x71D0A0` | ConsoleManager::Print(this, fmt, va_list) | NOT hooked (tee removed after a calling-convention crash) |
| `Script+0x28` | `Script::info.type` (u16): 0 object, 1 quest, 0x100 magic | used to skip quest-owned scripts on guests |
| `0x573170` | TESObjectREFR::Activate | NOT hooked: arg list unclear (reads `[ebp+8..+0x14]`), body ends in a jump table; see L5.md round 4 for the Intent design |

## 7. Dialogue menu map (static research for L8; none hooked yet; confidence M = prologue + call graph, semantics inferred)
| addr | name | evidence |
|---|---|---|
| `0x761A20` | `DialogMenu::Create(TESObjectREFR* speaker, ...)` static, cdecl | string "MENUS: DialogMenu::Create() called with a null speaker"; loads `Data/Menus/Dialog/dialog_menu.xml`; stores the new menu in `*0x11D9510` (so that address IS the singleton, now verified as a store target); vtbl slot 0x34 call afterwards. Hook here = "talk starts" with the speaker ref (arg1) |
| `0x762160` | `DialogMenu::HandleChoice()` static, cdecl | first test is `cmp [0x11D9510],0`; message "Dialog Menu: Invalid choice list encountered. Closing menu..."; called from vtbl-click handlers 0x762950 / 0x762ff0 and (via those) leads to `TESTopicInfo::RunResultScript` 0x61F170. "Topic/INFO chosen" |
| `0x764290` | `DialogMenu` destroy-if-open (static, cdecl, no args) | starts with `cmp [0x11D9510],0`, called from 0x5D8930. Best candidate for force-close (needs an in-game test: call it with a dialog open and no end-script side effects) |
| `0x7640A0` | topic-list builder (SEH frame) | calls `GetTopicInfo` wrapper 0x61A720 and 0x619D00; 5 callers (0x727430, 0x72D6D0, 0x75B750, 0x7ADA40, 0x7B78E0): the point where offered INFOs are resolved |
| `0x7624F0` | DialogMenu vtbl slot 3, `ret 8` | per-frame/event handler |
| `0x5C8740` | `Cmd_StartConversation`; `0x5C8A50` `Cmd_Say`; `0x5C9100` `Cmd_SayTo` | script-level conversation starters |
| `0x573170` | `TESObjectREFR::Activate(activator, u8, u32, u32)` thiscall, `ret 0x10` | 4 stack args at every call site (e.g. 0x5B5B18: `push 1; push 0; push 0; push activator`); body ends in a jump table. Hooked as an observer on Role::Guest (`setActivateObserver`) |
All calling conventions of hooked functions are now pinned at compile time (`PIN_F`/`PIN_C` static_asserts compare each detour's stack bytes with `HookSpec::retBytes` from `tests/retbytes.json`).

## 8. Added in round 11 (condition evaluators, EnableFastTravel; prologue-verified on the reference image)

Condition evaluators are the 4th column of the script command table (`command_table.json`: opcode, execute, parse, **eval**, ...). Signature
(read from the prologues and the caller `Condition::Evaluate` 0x681600): `bool eval(TESObjectREFR* thisObj, void* param1, void* param2, double* result)`,
cdecl, `ret` (caller pops 0x10). `Condition::Evaluate` itself is `thiscall(Condition*, subject, target, bool* flag)`, `ret 0xC`, returns the COMPARISON
result (bool), so the raw function value is only visible inside the eval handlers. The condition function index (`word [cond+8]`) is `opcode - 0x1000`
and indexes the eval table at `0x11908F8 + 0x28*idx` (handler at +0x10).

| address | function | conf | note |
|---|---|---|---|
| `0x59BFA0` | `GetDistance` eval | V | hooked (party-aware), stolen 7 |
| `0x59EF60` | `GetInCell` eval | V | hooked, stolen 6 |
| `0x59EE80` | `GetInSameCell` eval | V | hooked, stolen 6 |
| `0x5A3180` | `IsInCombat` eval | V | hooked, stolen 6 |
| `0x59D6E0` | `GetDetected` eval | V | hooked, stolen 6 |
| `0x59C7E0` | `GetHealthPercentage` eval | V | hooked, stolen 11 |
| `0x5D13E0` | `Cmd_EnableFastTravel` | V | hooked (capture -> SetFastTravelLock), stolen 6 |
| `0x5C09C0` | `Cmd_SetAngle` | V | NOT hooked yet: needs a rotation Effect type from L4 |
| `0x5CCB20` | MoveTo worker `(thisObj, marker, offX, offY, offZ)`, cdecl | M | called by `Cmd_MoveToMarker` 0x5CCF50 after ExtractArgs; destination = marker position + offsets |

## 9. Hit path (combat-hooks, 2026-10-21; derivation in docs/re/hit-path.md, constants in `plugin/combat/combat_addrs.h`)

| address | function | conf | note |
|---|---|---|---|
| `0x89A760` | ApplyHitData `thiscall(Actor* victim, ActorHitData*, bool)`, `ret 8` | M (name), V (bytes) | the engine's hit funnel; aligned-frame prologue `53 8B DC 83 EC 08 83 E4 F0 ...` (not hookable by the entry patcher); six `E8` callers 0x87C4DA 0x89A738 0x8B91E1 0x9B0503 0x9C1E96 0x9CBDE8, no other reference; reads `healthDmg [hit+0x14]` once. Call sites redirected by plugin/combat |
| `0x899CB0` | hit builder `ret 0x10`, standard prologue | M | allocates the 0x64-byte ActorHitData, constructs it, calls ApplyHitData at 0x89A738 |
| `0x9B4D90` | `ActorHitData::ctor` thiscall(this) -> this | V | `55 8B EC 51 89 4D FC 8B 4D FC 83 C1 38 E8 1E C8`; initialises every field; 0x9B4EC0 is the copy constructor |
| `0x1196DAC` | body-part category table (4-byte stride) | V | hitLocation 0..14 -> 2,1,1,3,3,4,4,5,5,5,5,5,5,1,6 (torso, head, left arm, right arm, legs, none) |

## 10. Story-capture command handlers (2026-10-21; `plugin/hooks/story_addrs.h`, generated, see docs/tools/story-capture.md)

74 script command handlers (73 ops of the `RefCommand` effect plus `Cmd_ForceActiveQuest`) hooked at their entry; four more ops (`StartCombat`, `StopCombat`, `Activate`, `ModFactionRank`) reuse the detours of the main table (`engine_addrs.h`). All are `bool __cdecl(pi, sd, thisObj, contObj, script, ev, res, opc)`, `ret`
(retBytes 0), conf V: the address is the `execute` column of the engine command table (`plugin/hooks/tools/re/command_table.json`), the first 16 bytes and the instruction-aligned stolen length
(5 to 11, never across a branch) come from a reference exe by `tools/re/pro.py`'s method and live in `plugin/hooks/tests/story_prologues_dump.txt`. Each command's parameter list (ParamInfo type id,
optional flag, `needsParent`) is in `plugin/hooks/data/story_cmd_params.json`. Data tables read from the image: animation group names `g_animGroupInfos` 0x11977D8 (stride 0x24, 245 entries,
the PlayGroup parameter is the index) and critical stage names `g_criticalStageNames` 0x119BBB0 (5 entries). Left out: `Cmd_ReleaseWeatherOverride` 0x5B76D0 (its prologue is
`push ebp; mov ebp, esp; call 0x46DD00`, a rel32 call inside the stolen bytes, which the patcher refuses). The four ownership commands (0x5B57E0, 0x5D6D10, 0x5D17F0, 0x5C7660) are hooked by the
world-time lane (`plugin/hooks_world/world_owner_addrs.h`), not here.

## 11. Party conditions, notes, kill attribution (P2, 2026-10-27; prologues and call graphs read from the reference image, `plugin/hooks/engine_addrs.h`)

Condition evaluators (same signature as section 8: `bool eval(thisObj, param1, param2, double* result)`, cdecl, `ret`). The column says which operand is "the player" and how a member's value is obtained (`plugin/hooks/party_cond.h` MemberView).

| address | function | conf | player operand / view | what the disassembly shows |
|---|---|---|---|---|
| `0x59F0C0` | `GetInWorldspace` eval | V | thisObj, ghost swap | param1 (form type 0x41 = worldspace) compared with the world of thisObj's parent cell (`0x575D70`) |
| `0x5A3270` | `IsInInterior` eval | V | thisObj, ghost swap | `0x8D6F30(thisObj)` = parent cell, `0x425FD0(cell)` = interior flag |
| `0x5A3F00` | `IsKiller` eval | V | param1, ghost swap | thisObj is the dead actor: IsActor (vtbl+0x100), IsDead (vtbl+0x22C), then `param1 == [actor+0xC0]` |
| `0x59DA90` | `GetEquipped` eval | V | thisObj, provider | IsActor (vtbl+0x100) on thisObj, item in param1, equip query through vtbl+0xE4 |
| `0x5A2F60` | `IsSneaking` eval | V | thisObj, provider | `0x4997B0(actor)` |
| `0x5A4630` | `HasPerk` eval | V | thisObj, provider | IsActor, then `vtbl+0x4A0(perk = param1, alt = param2 != 0)` returns the rank |
| `0x5A3BF0` | `GetHasNote` eval | V | none: reads the GLOBAL player, provider for the others | ignores thisObj, calls `PlayerCharacter::HasNote` `0x966C10` on `*0x11DEA3C` with param1 |
| `0x5A4820` | `GetPlayerTeammate` eval | V | thisObj is an NPC, owner view | IsActor, `0x566950(actor)` = the teammate flag (+0x18D, see `Actor::SetPlayerTeammate` 0x8BCA90) |
| `0x5CF490` | `IsPlayerInRegion` SCRIPT command (no condition evaluator: its eval column is 0) | V | none: reads `*0x11DEA3C + 0x764`, provider for the others | ExtractArgs (one region), `0x4F2770(player)` = `player+0x764` (the region list), then walks the region's parents; a ghost has no such list, so a swap cannot work |

Other:

| address | function | conf | note |
|---|---|---|---|
| `0x966A70` | `PlayerCharacter::AddNote(note, u8 verbose)` | V | thiscall, `ret 8`, hooked. The note list is `player+0x5E4`. Callers: `Cmd_AddNote` 0x5D5230, 0x4C37D0, 0x5E9360, 0x757F70, 0x892520 (the pickup / Pip-Boy paths), so hooking it captures every route, not only the script command. |
| `0x966B80` | `PlayerCharacter::RemoveNote(note)` | V | thiscall, `ret 4`; only caller is `Cmd_RemoveNote` 0x5D52A0 (stays captured at command level) |
| `0x966C10` | `PlayerCharacter::HasNote(note)` | V | thiscall, `ret 4`, not hooked; called by us to see whether an AddNote took |
| `0x5C8340` | `Cmd_RemoveFromAllFactions` | V | `needsParent`, no parameters; IsActor check, MarkModified (vtbl+0x48, 0x80000000), then clears the actor's runtime ExtraFactionChanges list (`0x5D43C0` = the actor's extra list, `0x42E800` = lookup, `0x42E760` = create). Captured as one `SetFactionRank{actor, faction, -1}` per faction the actor was in. |
| `0x89D900` | `Actor::Kill(victim, killer ref, float)` | V (name M) | aligned-frame prologue `53 8B DC ...`, NOT hookable by the entry patcher. Stores the killer pointer at `victim+0xC0` (`mov [ecx+0xC0], edx`, 0x89DAAC) before it raises the death. 13 callers (Cmd_KillActor 0x5BE2A0 among them). |
| `0x5BE2A0` | `Cmd_KillActor` | V | params (command table): Actor (optional, the KILLER), Integer, Integer. It calls `Actor::Kill(victim = thisObj, killer = the first parameter, 0.0)` at 0x5BE426, so the console line `KillActor <killerRef>` run on the victim makes the engine store the killer in its own order. `killWithAttribution` uses that. |
| `0x5B5690` | `Cmd_RemoveAllItems` | V | params: Container (optional, the destination), Ownership Added (optional int), No Message Flag (optional int); `needsParent` |

Offsets used by the plugin without a hook: killer slot `actor+0xC0` (above), region list `player+0x764`, note list `player+0x5E4`. Base factions come from the xNVSE layout (`TESActorBase::baseData.factionList`) and the runtime list from `ExtraFactionChanges` (extra type 0x5E): xNVSE sources (S), not re-derived.

Needs in-game proof (none of the above has run in a game): that the ghost reference answers `GetInWorldspace` / `IsInInterior` / `IsKiller` (it has a parent cell, and Actor+0xC0 compares equal to the ghost pointer); that `KillActor <ghostHex>` is accepted by the console and the victim's OnDeath block then sees `IsKiller` true for that ghost; that a looted note object ends in 0x966A70 on the machine that looted it (and with `verbose` as the engine passes it); that `RemoveFromAllFactions` lists exactly the factions `GetInFaction` reports before the call. Also: loading a save must not make the AddNote hook emit transactions (the engine should restore the note list without calling 0x966A70); a guest's own pickup of the Jessup note reaches the observer once and the host's GetHasNote is true afterwards; `IsPlayerInRegion` needs a provider that knows each member's region, without one it keeps the local answer.

## 12. Wave 1b condition functions (fixer w1b, 2026-10-27; evaluator addresses are the eval column of `out/L5/re/command_table.json`, prologues and call graphs read from the reference image)

Same evaluator signature as sections 8 and 11. All six are hooked in `plugin/hooks/engine_hooks.cpp` (`COND_HOOK`) and listed in `plugin/hooks/engine_addrs.h`, `tests/prologues_dump.txt` and `tests/retbytes.json` (ret 0, cdecl).

| address | function | conf | player operand / view | what the disassembly shows |
|---|---|---|---|---|
| `0x5A00C0` | `GetLevel` eval | V | thisObj, provider | IsActor (vtbl+0x100) on thisObj, then `0x87F9F0(actor)` (the level, a word returned in ax, converted to the result double). **`Cmd_GetLevel` `0x5C84D0` is `call 0x5A00C0(thisObj, 0, 0, result)`** (0x5C84DF), so this one detour covers `Player.GetLevel` in scripts as well as the condition. |
| `0x59F3C0` | `GetIsSex` eval | V | thisObj, provider | `0x7AF430(thisObj)`, `0x401170` (form type, compared with 0x2A = NPC), `0x5F0CC0` (the sex), compared with param1 (an integer). |
| `0x59F770` | `GetPCIsSex` eval | V | none: reads the GLOBAL player `*0x11DEA3C`, provider | pushes (player, param1, param2) and calls the GetIsSex body; thisObj is ignored. |
| `0x59DD90` | `GetSitting` eval | V | thisObj, provider | IsActor, then vtbl+0x214 (sit state) mapped through a 5-entry jump table. |
| `0x59ED30` | `GetShouldAttack` eval | V | param1 (the target), ghost swap | thisObj is the NPC (IsActor), param1 must be an ACHR / ACRE (`0x401170` returns 0x3B or 0x3C), the NPC's own AI decides. |
| `0x5A3B20` | `IsPlayerActionActive` eval | V | none: reads the GLOBAL player, provider | `0x963CE0(*0x11DEA3C, param1)` (the log string is "IsActionInList"). |

Not hooked, with the reason (decided in `party_cond.cpp` `condCoverage`, walked against every `PartyAwareCond` function name in rules.json by a test):

| function | eval column | why |
|---|---|---|
| `GetActionRef` `0x5CE360`, `IsActionRef` `0x5B5950` | 0 (script commands only) | the activator is the acting player of the intent: `coop_module.cpp` runs a guest's activation as the host's `0x14`, so `IsActionRef player` already holds for the guest who acted. |
| `GetIsReference` `0x59FBB0` | `0x59FBB0` | a reference identity test (thisObj == param1); "the player" is the same reference for every member. |
| `GetMapMarkerVisible` `0x5A51E0`, `GetTalkedToPC` `0x59E0F0` | set | a flag on the marker / on the NPC, no player operand. |
| `GetButtonPressed`, `EnterTrigger` | n/a | the script's own box (UiRouter answers it) and the trigger event. |

Needs in-game proof (none of the above has run in a game): that the six detours install (the prologues match the image only on paper until `Init` reports them); that `Player.GetLevel` in an XP-tier script reaches the `GetLevel` detour (the direct `call 0x5A00C0` is on the image, a detour at the entry catches it); that a ghost reference answers `GetShouldAttack` (the NPC's AI evaluates the ghost as a target); that a StateProvider answer for `GetIsSex` / `IsPlayerActionActive` is reduced as `initiator` (their only rules). `KillActor` of a ghost ref (`player_kill.h`): that the console accepts it and that `ModActorValue Health -100000` from the rules engine kills the receiving machine's own player.

## 13. Worker threads, the InterfaceManager update and the VATS highlight (2026-10-27; `plugin/vats/vats_addrs.h`, read from the reference image, the crash log and a hang minidump)

| address | what | conf | notes |
|---|---|---|---|
| `0x8C7290` | creates the AI linear task threads | V | more than 2 cores: "AI Linear Task Thread 1" (`0x8C7DA0`) and "... 2" (`0x8C7F50`); else one "AI Linear Task Thread" (`0x8C7BD0`). Events: group 0 (12, progress of the workers), group 1 (8, main to workers), all manual-reset |
| `0x8C78C0` | the kick (main loop `0x86EC87`), thiscall on `0x11DFA50` | V | resets every event, zeroes the progress counters, byte `0x11DFA19` = 1, resumes the threads; only when the world is not paused (`0x11DEA2B`, `0x11DEA2D`, `0x11DEA2E`, main+7 all 0) |
| `0x8C7990` | the join (main loop `0x86EE4E`) | V | byte `0x11DFA19` = 0 |
| `0x8C79E0` / `0x8C7A70` | SetEvent(group, i) and record i / WaitForSingleObject(group, i, INFINITE) | V | |
| `0x86FD70` | InterfaceManager per-frame update | V | `0x7027E0`, `0x702810` (-> `0x70C4A0`), `0x702840`. Stage 0 of the worker (`0x8C7DB9`, `0x8C7BE9`), then event (0, 1). A paused frame: the main thread calls it (`0x86ECBA`); one core: `0x86EB36` |
| `0x713C70` | menu update | V | `if (cores > 1 && 0x713D90()) wait (0, 1)` at `0x713C8D` / `0x713CA4`; reached from the render path `0x870A00` / `0x870BD0` -> `0x871290` -> `0x871A50` (`0x871C77`) |
| `IM+0x1DC` | VATSHighlightData (accessor `0x705910`: IM if IM->byte0, + 0x1DC) | V | +0 mode, +4 target ref, +8 NiPointer to the target's 3D, +0xC count, +0x10 index, +0x14 [32] {ref, 3D}, +0x118 flag. Set target `0x800AC0(ref, limb, flag)` (writes +4 then +8), add `0x800D50`, clear list `0x800ED0`, per-frame `0x800370` (clears the target when `vtbl+0x22C` IsDead says so, `0x800495`; takes the renderer lock at `0x800505`) |
| `0x800F30` | draw the highlight (thiscall, 6 args) | V | reads +4 without a check (`0x800F66`); callers `0x8749B0` / `0x874AC0` check +8 first; `0x873200` calls it again at `0x8741AF` on the earlier check's result, several ms later |

The game never kicks the workers with a menu open, so the update and the highlight draw never overlap. Real-time VATS does (the world runs under the VATS menu): waited for, the worker's update deadlocked on the renderer lock held by the main thread's highlight draw; not waited for, it cleared +4 while the main thread drew, the crash at `FalloutNV.exe+400F66`. plugin/vats now runs the update on the main thread before the kick while a menu is open.

## 14. INFO conditions, GetItemCount / GetActorValue script commands, RemoveAllTypedItems (W3d, 2026-10-28; read from the reference image with capstone, `plugin/hooks/tools/check_cmd_routing.py` re-checks the call edges)

| address | what | conf | notes |
|---|---|---|---|
| `0x61E600` | `TESTopicInfo::CheckConditions`, thiscall(info, a1, a2, a3, a4), `ret 0x10` | M (prologue `55 8B EC 83 EC 14 56 89 4D EC 8B 4D EC E8`, 6 bytes stolen; the name is inferred) | the one function that evaluates an INFO's condition list. It tests the INFO flags (`+0x26`), then calls `ConditionList::Evaluate` `0x680C60` on the list from `0x5F5760` and a second list from `0x500940`; a4 is the speaker (`IsActor` through vtbl+0x100). Callers: `GetTopicInfo` `0x61A7D0` (through `0x61A790`) and `0x61AF30` (called from the dialogue click handler `0x762FF0`). Hooked (`h_InfoConditions`): a `ConditionQuestScope` for the INFO's quest around the original, so the Jed weight gate and the Shaman's root count reduce under their quest's PartyAwareCond rule |
| `0x5C6A70` | `Cmd_GetItemCount` | V | `ExtractArgs` `0x5ACCB0`, then `call 0x59D8E0` (the condition evaluator `Cond_GetItemCount`) at `0x5C6ABC` with `[ebp+0x10]` (the command's thisObj) as the first operand. The entry detour on `0x59D8E0` therefore sees the command too: `Player.GetItemCount X` in a quest script is reduced over the party by `condEval` |
| `0x5BD8A0` | `Cmd_GetActorValue` | V | the same shape: `call 0x59C4F0` (`Cond_GetActorValue`) at `0x5BD8EC`, thisObj first. `Player.GetAV InventoryWeight` in a script is the max over the party where the quest's rule says so |
| `0x5B55A0` | `Cmd_RemoveAllTypedItems`, command table entry `0x1196478` | V | parameters (read from the table, `0x118E170`): Container (optional ref, type 26), Ownership Added (int), No Message Flag (int), Type (int, a form type: 40 = TESObjectWEAP, 24 = TESObjectARMO), Holdout List (optional form list, type 37). `RemoveAllItems` `0x5B5690` has the first three |

`h_GetItemCountCmd` / `h_GetAVCmd` (the `PlayerFactsScope` answer for a guest's INFO result script) are in front of the original command; with no facts scope they forward, and the original reaches the detoured evaluator. W3b request 7 ("the commands do not call the reducer") was a reading of the command hooks alone.

Needs in-game proof: that `TESTopicInfo::CheckConditions` installs (the prologue matches the image on paper only) and that `status.w3d.infoScopesOpened` rises when a dialogue menu is built; that the per-slot re-run of a script (`status.w3d.perSlotHookRuns`) really sees `player` as the guest's puppet (`ActingPlayerScope`), which is the same mechanism a guest's activation already relies on.
